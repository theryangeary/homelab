#!/bin/sh

if [ -z $FTP_USERNAME ]; then
    echo "FTP_USERNAME not set"
    exit 1
fi

if [ -z $FTP_PASSWORD_FILE ]; then
    echo "FTP_PASSWORD_FILE not set"
    exit 1
fi

FTP_PASSWORD="$(cat $FTP_PASSWORD_FILE)"

mkdir -p /backup
cd /backup
BACKUP_FILE="backup.tar.gz"

tar czvf $BACKUP_FILE /data

ftp -n $FTP_HOST 21 <<END_SCRIPT
user ${FTP_USERNAME} ${FTP_PASSWORD}
cd backups
put $BACKUP_FILE
quit
END_SCRIPT


