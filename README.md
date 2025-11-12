## 概要
このリポジトリは、Rust + Bevyで gifファイル を指定して動かすツールの検証リポジトリになります。
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
* gifが表示されたら　移動、スケール、delay, x-flipでgif ファイルを編集できます。
### gif を削除
* -ボタンを押すと項目ごとgifを削除します
### 設定ウィンドウの表示、非表示
Escキー
### ウィンドウのデコレーションを非表示にして、背景色を透明にする
F12キー

## 動作確認
Windows 11
