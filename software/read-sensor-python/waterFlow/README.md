Create bash script file

```
sudo nano launcher.sh
```

then enter this

```shell
#!/bin/bash

cd ~/iotadd-pwo/software/read-sensor-python/waterFlow
python3 waterFlowMeter.py 
```

Save and close

Now change the permissions of the file

```
sudo chmod 755 launcher.sh
```

then add a scheduled task with cron, type the following command

```
crontab -e
```

will ask to choose a text editor, we will type 1 to choose the
nano text editor

![](./editor.png)

will open a file where at the end of all the text
we will add the following

```
@reboot /bin/sleep 60 ; ~/iotadd-pwo/software/read-sensor-python/waterFlow/launcher.sh
```

Save and close

Run the following command to verify that they have been
saved the changes

```
crontab -l
```

reboot device

```
sudo reboot
```

Find the line where it tells us that our file
Python is running

```
ps -aux | grep python3
```

Done