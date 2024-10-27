this crate provides bayou types by default as well as
- cryptography logic enabled with the `crypto` feature
- activitypub and versia protocol logic enabled with the `protocol` feature. this depends upon `crypto`

the sign string parsing in this crate borrows heavily from https://github.com/astro/sigh 

this project also tries to make use of the documentation from mastodon however it is quite poor
 - https://docs.joinmastodon.org/spec/security/#http-sign
 - https://blog.joinmastodon.org/2018/07/how-to-make-friends-and-verify-requests/
 - https://docs.joinmastodon.org/spec/activitypub/#publicKey

this crate aims to have extensive unit testing to ensure types map to real world activitypub objects, however the protocol logic is currently untested and will need to be performed by the [bayou_server](/server)