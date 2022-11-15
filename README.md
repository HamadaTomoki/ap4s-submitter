# ap4s-submitter
AP4Sの応用午前フォームの提出を半自動化します。

## セットアップ
Rustのインストール  
https://doc.rust-jp.rs/book-ja/ch01-01-installation.html

## デモ
https://www.dropbox.com/s/ck9xrclqqyjevm1/ap4s-submitter.mp4?dl=0

## 操作方法

1. ターミナルを開き、プロジェクト直下に移動
2. `cargo run`で実行
3. フォームのリンクを入力
4. 実行中、見つからなかった回答があれば、開かれたブラウザから検索し、その都度、手動で入力
5. 回答が終了後、開かれたフォームで回答がしっかりと入力されているか確認  
   入力漏れがあれば、ターミナルに表示された結果と照らし合わせてフォームで回答
6. フォームで送信ボタンを押す
7. ターミナルでEnterを押してプログラムを終了

※ main.rsの`HEADLESS`定数を`true`にすることでバックグラウンドで実行が可能です。ブラウザを起動しない分、正確で速度も向上しますが、スコアを表示できないことに注意してください。

