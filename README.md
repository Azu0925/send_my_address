# send_my_address
自身のアドレスなどの情報をdiscord webhookに送信するcliです。

## コマンド
```sh
./send_my_address -w {your webhook url}
```

## 使い方
1. ラズパイの好きなディレクトリに実行ファイルを配置してください。
2. `/etc/rc.local` に上記コマンドを追加する。（パスとファイル名、URLは自分のものに合わせてください。）
3. 有線LANを接続するか、自動接続ができるWi-Fiがある環境で再起動する。

## その他
- 現在releaseにあるのはラズパイdebian系のもののみです。
- それ以外は自身の環境でビルドしてください。
