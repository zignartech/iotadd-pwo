from sensor.SHT20 import SHT20
import sys
import json
# I2C bus=1, Address=0x40
sht = SHT20(1, 0x40)

h = sht.humidity()  # read humidity

t = sht.temperature()  # read temperature

ambient_jsn = {"Temperature":round(t.C,2),"Humidity":round(h.RH,2)}
send_jsn = json.dumps(ambient_jsn)

sys.stdout.write(send_jsn)
h, t = sht.all()  # read both at once
