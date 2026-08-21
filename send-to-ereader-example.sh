#!/bin/bash

#transferring file
HOST='EREADER_IP_GOES_HERE'
USER='root'
FILE='img.raw'
REMOTEPATH='/tmp'

notify-send "starting transfer"

ftp -n $HOST <<END_SCRIPT
quote USER $USER
cd $REMOTEPATH
put $FILE
quit
END_SCRIPT

notify-send "transferred, starting to display"

# displaying file
nc $HOST 23 << END
root
/usr/bin/dr
pkill nickel
exit
END

notify-send "done displaying!"
exit 0


