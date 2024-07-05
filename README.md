
# Flashout

&nbsp;&nbsp;&nbsp;Auto redeem airtime from flashout.

**what it is:** A session of a puppet browser.

**what it is not:** a hack or an exploit

###

## Environment Variables

To run this project, you will need to add the following environment variables to your `.env` file, can be copied from `.env-example`

`USER_AGENT` - could be dynamic

`EMAIL` - the email for the account you want to redeem from

`PASSWORD` - the password for the account

`CURRENCY` - your currency letters, 

`COUNTRY` - country two letter name, eg: KE, TZ, UG


## CMDLine Arguments

`-a`, `--action` - the action you want to perform, **redeem** or **withdraw** recommended

`-c`, `--credit-amount` - amount you wish to redeem, (*5*, *10*, *20*, *35*, *50* - kenyan ofc)

`-n`, `--network` - **saf**, **safaricom** or **airtel**

`-u`, `--user-interface` - include this argument to spawn a puppet chrome process

`--proxy` - use a proxy server for the connection

`-p`, `--phone` - the phone number you want to redeem to in international format excluding the **+**
## Building

release
```bash
  git clone https://github.com/rotleaf/flashout.git
  cd flashout
  make release
```
debug
```bash
  git clone https://github.com/rotleaf/flashout.git
  cd flashout
  make debug
```

## Run

redeem *50* to *+25411187658* of network *safaricom*
```bash
  ./flashout -a withdraw -c 50 -p 25411187658 -n safaricom
```




## Contributing

Contributions are always welcome!

