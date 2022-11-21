mod enums;
pub mod model;

use std::sync::Arc;
use std::{thread::sleep, time::Duration};

use enums::xpath::{google_form, siken_dot_com};
use enums::Answers;
use headless_chrome::{Browser, Element};
use model::Student;
use regex::Regex;
use text_io::read;
use urlencoding::encode;

pub struct Crowling {
    browser: Browser,
}

#[allow(unused_must_use)]
impl Crowling {
    pub fn new(browser: Browser) -> Self {
        Crowling { browser }
    }

    pub fn crowl(&self, form_url: &str, student: &Student) -> anyhow::Result<()> {
        println!("\nNow crawling...");

        let tab = self.browser.wait_for_initial_tab()?;

        // navigate to the google form website.
        tab.navigate_to(form_url);
        tab.wait_until_navigated();

        // type the student infomatiまりはこon.
        self.type_student_info(student)?;

        // type the collects.
        let answers = self.type_answers()?;
        println!("\n-- Result --");
        for (i, ans) in (0_i32..).zip(answers.iter()) {
            println!("{}. {}", i + 1, ans,);
        }

        // submit
        println!("\n-- Please go to Google form and press the execute button! --");
        println!("\nAfter submiting, enter and quite.");
        let _: String = read!("{}\n");

        Ok(())
    }

    pub fn close(&self) -> anyhow::Result<()> {
        self.browser.wait_for_initial_tab()?.close_with_unload()?;
        Ok(())
    }

    fn type_student_info(&self, student: &Student) -> anyhow::Result<()> {
        let type_an_student_info = |stu_info: &str, stu_xpath: String| -> anyhow::Result<()> {
            let tab = self.browser.wait_for_initial_tab()?;
            let element = tab.find_element_by_xpath(&stu_xpath)?;
            element.type_into(stu_info);
            Ok(())
        };
        type_an_student_info(&student.class_id, google_form::Student::ClassId.to_string())?;
        type_an_student_info(&student.id, google_form::Student::Id.to_string())?;
        type_an_student_info(&student.name, google_form::Student::Name.to_string())?;
        Ok(())
    }

    fn type_answers(&self) -> anyhow::Result<Vec<String>> {
        let answers = self.get_answers()?;
        for (i, ans) in (0_i32..).zip(answers.iter()) {
            match ans.to_owned() {
                ans if ans == Answers::A.to_string() => {
                    self.click_element(google_form::Answers::A(i).to_string())?;
                }

                ans if ans == Answers::I.to_string() => {
                    self.click_element(google_form::Answers::I(i).to_string())?;
                }

                ans if ans == Answers::U.to_string() => {
                    self.click_element(google_form::Answers::U(i).to_string())?;
                }

                ans if ans == Answers::E.to_string() => {
                    self.click_element(google_form::Answers::E(i).to_string())?;
                }
                _ => {}
            }
        }
        Ok(answers)
    }

    fn click_element(&self, xpath: String) -> anyhow::Result<()> {
        let tab = self.browser.wait_for_initial_tab()?;
        let element = tab.find_element_by_xpath(&xpath)?;
        element.scroll_into_view();
        sleep(Duration::from_millis(100));
        element.click();
        Ok(())
    }

    fn get_answers(&self) -> anyhow::Result<Vec<String>> {
        let rm_symbol = |ans: &str| -> String {
            let mut answer = ans.to_owned();
            answer.retain(|c| !r#"()ーも有用はて、，,？?。・ .;:"#.contains(c));
            answer
        };
        let browser = Browser::default()?;
        let search_tab = browser.wait_for_initial_tab()?;

        let questions = self.get_questions();
        let urls_of_question = questions
            .iter()
            .map(|ans| {
                (
                    ans.to_owned(),
                    self.find_website_links(
                        &enums::Url::GoogleSearch(&encode(ans)).to_string(),
                        &search_tab,
                    )
                    .unwrap(),
                )
            })
            .collect::<Vec<(String, Vec<String>)>>();
        search_tab.close_with_unload();

        let mut collects = Vec::new();

        let browser = Browser::default()?;
        let siken_tab = browser.wait_for_initial_tab()?;

        'top: for (i, uoq) in (0_i32..).zip(urls_of_question.iter()) {
            let collect_cnt = collects.len();
            for url in uoq.1.iter() {
                siken_tab.navigate_to(url);
                siken_tab.wait_until_navigated();

                let ans = self.get_node_value(
                    &siken_tab
                        .find_element_by_xpath(&siken_dot_com::Question::Answer.to_string())?,
                );
                if rm_symbol(&ans) == rm_symbol(&uoq.0) {
                    let collect = self
                        .get_node_value(&siken_tab.find_element_by_xpath(
                            &siken_dot_com::Question::Collect.to_string(),
                        )?);
                    println!("{}. {}: {}", i + 1, ans, collect);
                    collects.push(collect);
                    continue 'top;
                }
            }

            if collects.len() != collect_cnt + 1 {
                println!("\n【Answer is not found !】 \n-- Please search in the browser and choise an answear from the following numbers one to four. --\nA browser with keywords searched from the question title will open...\n\n[Title]: {}", &uoq.0);
                // webbrowser::open(&enums::Url::GoogleSearch(&uoq.0).to_string());

                loop {
                    println!("-- Please select and type a number from the following. --\nex). 1\n   1. ア\n   2. イ\n   3. ウ\n   4. エ");

                    let input: String = read!("{}\n");
                    let input = input.trim().parse::<i32>().unwrap_or(0);
                    match input {
                        1 => {
                            collects.push(Answers::A.to_string());
                            break;
                        }
                        2 => {
                            collects.push(Answers::I.to_string());
                            break;
                        }
                        3 => {
                            collects.push(Answers::U.to_string());
                            break;
                        }
                        4 => {
                            collects.push(Answers::E.to_string());
                            break;
                        }
                        _ => {
                            println!("-- Please type '1' to '4' here again. --\nYou should type an answer that is not in the options.");
                        }
                    }
                }
            }
        }
        siken_tab.close_with_unload();
        Ok(collects)
    }

    fn find_website_links(
        &self,
        url: &str,
        search_tab: &Arc<headless_chrome::Tab>,
    ) -> anyhow::Result<Vec<String>> {
        search_tab.navigate_to(url);
        search_tab.wait_until_navigated();

        let rx = Regex::new(r#"^https?://(|www)\...-siken\.com.*$"#)?;
        let els = search_tab.find_elements("a")?;
        let attrs = els.iter().map(|x| x.get_attributes().unwrap().unwrap());
        let mut urls = vec![];
        attrs.for_each(|url| {
            url.iter().for_each(|x| {
                if rx.is_match(x) {
                    urls.push(x.to_owned());
                }
            })
        });
        Ok(urls)
    }

    fn get_questions(&self) -> Vec<String> {
        let tab = self.browser.wait_for_initial_tab().unwrap();
        let element = tab.find_elements_by_xpath(google_form::QUESTIONS).unwrap();
        element.iter().map(|el| self.get_node_value(el)).collect()
    }

    fn get_node_value(&self, el: &Element) -> String {
        el.get_description().unwrap().children.unwrap()[0]
            .node_value
            .to_owned()
    }
}
