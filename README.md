
# Flashout

&nbsp;&nbsp;&nbsp;&nbsp;Auto redeem airtime from flashout.

&nbsp;&nbsp;&nbsp;&nbsp;**what it is:** A session of a puppet browser.

&nbsp;&nbsp;&nbsp;&nbsp;**what it is not:** a hack or an exploit

###

## Environment Variables

&nbsp;&nbsp;&nbsp;&nbsp;To run this project, you will need to add the following environment variables to your `.env` file, can be copied from `.env-example`

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`USER_AGENT` - could be dynamic

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`EMAIL` - the email for the account you want to redeem from

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`PASSWORD` - the password for the account

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`CURRENCY` - your currency letters, 

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`COUNTRY` - country two letter name, eg: KE, TZ, UG


## CMDLine Arguments

&nbsp;&nbsp;&nbsp;&nbsp;`-a`, `--action` - the action you want to perform, **redeem** or **withdraw** recommended

&nbsp;&nbsp;&nbsp;&nbsp;`-c`, `--credit-amount` - amount you wish to redeem, (*5*, *10*, *20*, *35*, *50* - kenyan ofc)

&nbsp;&nbsp;&nbsp;&nbsp;`-n`, `--network` - **saf**, **safaricom** or **airtel**

&nbsp;&nbsp;&nbsp;&nbsp;`-u`, `--user-interface` - include this argument to spawn a puppet chrome process

&nbsp;&nbsp;&nbsp;&nbsp;`--proxy` - use a proxy server for the connection

&nbsp;&nbsp;&nbsp;&nbsp;`-p`, `--phone` - the phone number you want to redeem to in international format excluding the **+**
## Building

&nbsp;&nbsp;&nbsp;&nbsp;release
```bash
git clone https://github.com/rotleaf/flashout.git
cd flashout
make release
```
&nbsp;&nbsp;&nbsp;&nbsp;debug
```bash
git clone https://github.com/rotleaf/flashout.git
cd flashout
make debug
```

## Run

&nbsp;&nbsp;&nbsp;&nbsp;redeem *50* to *+25411187658* of network *safaricom*
```bash
./flashout -a withdraw -c 50 -p 25411187658 -n safaricom
```


## Contributing

&nbsp;&nbsp;&nbsp;&nbsp;Contributions are always welcome!

