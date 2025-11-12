## 概要
このリポジトリは、Rust + Bevyで URLからgifファイルを動かすツールのリポジトリになります。
![bbb](https://github.com/user-attachments/assets/924af331-394b-4880-986d-fe4604a2502f)


## ビルド
[Rust](https://rust-lang.org/ja/tools/install/)をインストール後に
```
git clone https://github.com/8bitTD/gif_bevy
cd gif_bevy
cargo run --release
```

## 使い方
### gif を追加する
* 設定ウィンドウの＋ボタンを押して項目を追加する
* urlをクリックしてモーダルにgifのURLを入力して、読み込みボタンを押す
### gif を編集する
* gifが表示されたら　移動、スケール、delay、x-flip でgifファイルを編集できます。

　<img width="324" height="68" alt="image" src="https://github.com/user-attachments/assets/2a983f35-002f-409d-bad7-0293d028ac54" />

### gif を削除
* -ボタンを押すと項目ごとgifを削除します
### 設定ウィンドウの表示、非表示
Esc キー
### ウィンドウのデコレーションを非表示にして、背景色を透明にする
F12 キー

## 動作確認
Windows 11
