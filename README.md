# ap4s-submitter
AP4Sの応用午前フォームの提出を半自動化します。

## 実行ファイル

- [Windows](https://github.com/HamadaTomoki/ap4s-submitter/releases/download/latest/ap4s-submitter_null_x86_64-pc-windows-gnu.zip)
- [MacOS](https://github.com/HamadaTomoki/ap4s-submitter/releases/download/latest/ap4s-submitter_null_x86_64-apple-darwin.zip)
- [Linux](https://github.com/HamadaTomoki/ap4s-submitter/releases/download/latest/ap4s-submitter_null_x86_64-unknown-linux-musl.zip)

## 使い方

1. 実行ファイルをダウンロード
2. zipファイルを解凍して、実行ファイルをクリック(MacOSの方は、Ctrlを押しながらクリックしてください。)
3. `Headless Mode`を`n`に指定   
	※ `Headless Mode` を`Y`にすることでバックグラウンドで実行が可能です。ブラウザを起動しない分、クリックが正確で速度も向上しますが、スコアを表示できないことに注意してください。
4. 学生情報を入力(クラス記号, 学籍番号, 名前)
5. フォームのリンクを入力
6. しばらく、クローリングされるのを待つ(すべての情報を取得するので少し時間がかかります。)
7. 実行中、見つからなかった解答があれば、開かれたブラウザから検索し、その都度、手動で入力
8. 解答が終了後、開かれたフォームで解答がしっかりと入力されているか確認  
   入力漏れがあれば、ターミナルに表示された結果と照らし合わせてフォームで解答
9. フォームで送信ボタンを押す
10. ターミナルでEnterを押してプログラムを終了


