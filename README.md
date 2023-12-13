# Icpbin

IcpBin is an alternative for PasteBin and is aimed to a tutorial for developers who want to learn more about the Internet Computer and how develop their apps with Rust.

## What is IcpBin?
some time we want to save a note or a code snippet and share between device, but we don't access the device. for example we are in work computer and share some setting with personal computer or i want to show me note to my audience. in this time we want to sure that this data won't change by any body and live long as i want.
icpbin came to solve exact this type of problems. in icpbin you can create note (that we called Paste) and share between everyone. even without signin.
Paste can have name and  tags to customize it. even every paste has expire time that after that time paste will be deleted.

## Where are features like password encryption and categories in IcpBin?
in case of categories i think that it is not necessary because we can use tags for that.tags are greate for customize and search.client can append own specific info to tags to track their Pastes.but in case of password encryption i think that it is not necessary, because icpbin is like backend and people won't use it without client, and client can encrypt data before send it to server, also it is better to won't share your sensitive data in public blockchain that everybody can see.



## Running the project locally
If you want to test your project locally, you can use the following commands:

```bash
cd icpbin/
dfx start --background
```
then run 
```bash
yarn gen-deploy
```
if you gen permission error you can run

```bash
chmod +x did.sh
```

then rerun 
```bash
yarn gen-deploy
```


> i created this project for first challenge my self and also participate in [dacade.org internet Computer
Rust Smart Contract 101](https://dacade.org/communities/icp/challenges/c04ec537-c4a7-4681-9c62-2b7571d55a5e). fell free to check it out.