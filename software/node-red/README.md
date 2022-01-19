# Node-RED - Running on Raspberry Pi

## Running as a service

The following commands are provided to work with the service:

* `node-red-start` - this starts the Node-RED service and displays its log output. Pressing Ctrl-C or closing the window does not stop the service; it keeps running in the background
* `node-red-stop` - this stops the Node-RED service
* `node-red-restart` - this stops and restarts the Node-RED service
* `node-red-log` - this displays the log output of the service

You can also start the Node-RED service on the Raspberry Pi OS Desktop by selecting the `Menu -> Programming -> Node-RED` menu option.

Autostart on boot

If you want Node-RED to run when the Pi is turned on, or re-booted, you can enable the service to autostart by running the command:
```
sudo systemctl enable nodered.service
```
To disable the service, run the command:

```
sudo systemctl disable nodered.service
```

#### For the project we recommend running the following command in the console

```
sudo systemctl enable nodered.service
```

## Opening the editor

Once Node-RED is running you can access the editor in a browser.

If you are using the browser on the Pi desktop, you can open the address: http://localhost:1880.

When browsing from another machine you should use the hostname or IP-address of the Pi: `http://<hostname>:1880`. You can find the IP address by running hostname -I on the Pi.

![](./images/nodered.png)

## Install Nodes Using Node-RED UI

Perform below listed steps to search and add a node to your node-RED instance.

* Click on the menu icon (3 lines) on top right corner of the page.
* Now, click on Manage palette option
* Nodes tab will show all the palettes / nodes you have installed.
* Click on Install tab. This is where we can search for wide range of nodes from the community.
* Type the search query and you will have the matching results.
* Click on install button and the confirmation button of the node you want to be installed.
* This will install the node in Node-RED.

#### Install two required palettes

* Menu and Manage palette

![](./images/manage_palette.png)

* The palettes and Click on Install tab

![](./images/palettes.png)

* Type the search query, install and confirmation

1.  Install node-red-contrib-ezo
![](./images/node-red-contrib-ezo.png)
    - Click on install
![](./images/install_ezo.png)

2.  Install node-red-contrib-influxdb
![](./images/node-red-contrib-influxdb.png)
    - Click on install
![](./images/install_influxdb.png)

* Installed Nodes.
![](./images/installed_nodes.png)

## Importing Flows

Flows can be imported and exported from the editor using their JSON format, making it very easy to share flows with others.

#### Importing required flows

The Import dialog can be used to import a flow by the following methods:

* Click on the menu icon (3 lines) on top right corner of the page.
* Now, click on Import option

![](./images/import.png)
* Click on Select a file to Import
![](./images/select_file.png)
* Select the file flows.json and upload
![](./images/file_upload.png)
* Click on Import
![](./images/import_file.png)
* Then, click on Deploy
![](./images/deploy.png)


