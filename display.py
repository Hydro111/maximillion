import sys
import struct
import plotly
import os
import plotly.graph_objects as go
import math as m

class Frame:
	xE:list[float]
	yE:list[float]
	zE:list[float]
	uE:list[float]
	vE:list[float]
	wE:list[float]
	xB:list[float]
	yB:list[float]
	zB:list[float]
	uB:list[float]
	vB:list[float]
	wB:list[float]

	def __init__(
			self, 
			xE:list[float], yE:list[float], zE:list[float], uE:list[float], 
			vE:list[float], wE:list[float], xB:list[float], yB:list[float], 
			zB:list[float], uB:list[float], vB:list[float], wB:list[float]
		) -> None:
		self.xE = xE
		self.yE = yE
		self.zE = zE
		self.uE = uE
		self.vE = vE
		self.wE = wE
		self.xB = xB
		self.yB = yB
		self.zB = zB
		self.uB = uB
		self.vB = vB
		self.wB = wB


raw_data = sys.stdin.buffer.read()
#with open("test1.dat", "rb") as f:
#	raw_data = f.read()
print(f"Transfering and displaying... ({len(raw_data)} bytes)")
simulation_data = raw_data[8:]
#print(raw_data[-5:])
#print(raw_data[90800:90840])

latice_density:float = struct.unpack("f", raw_data[:4])[0]

latice_spacing:float = 1.0/latice_density
dt:float = struct.unpack("f", raw_data[4:8])[0]
#print(len(simulation_data))
#print(latice_density)

frames:list[Frame] = []

uTemp:float = 0
vTemp:float = 0
wTemp:float = 0

xE:list[float] = []
yE:list[float] = []
zE:list[float] = []
uE:list[float] = []
vE:list[float] = []
wE:list[float] = []

xB:list[float] = []
yB:list[float] = []
zB:list[float] = []
uB:list[float] = []
vB:list[float] = []
wB:list[float] = []

x_current:float = 0.0
y_current:float = 0.0
z_current:float = 0.0

i_current:int = 0

targeting_E:bool = True


for i in range(0, len(simulation_data), 5):
	packet = simulation_data[i:i+5]
	if len(packet) != 5: 
		print(len(packet))
		print(i)
	value = struct.unpack("f", packet[:4])[0]
	deliminator = packet[4]
	#print(f"{i}: {value}; {deliminator}")
	
	if targeting_E:
		if i_current == 0:
			uTemp = value
		elif i_current == 1:
			vTemp = value
		else:
			if abs(uTemp) + abs(vTemp) + abs(value) > 0.01:
				xE.append(x_current)
				yE.append(y_current)
				zE.append(z_current)
				uE.append(uTemp)
				vE.append(vTemp)
				wE.append(value)
	else:
		if i_current == 0:
			uTemp = value
		elif i_current == 1:
			vTemp = value
		else:
			if abs(uTemp) + abs(vTemp) + abs(value) > 0.01:
				xB.append(x_current)
				yB.append(y_current)
				zB.append(z_current)
				uB.append(uTemp)
				vB.append(vTemp)
				wB.append(value)
	
	i_current += 1
	i_current %= 3

	if deliminator >= 1:
		targeting_E = not targeting_E
	if deliminator >= 2:
		x_current += latice_spacing
	if deliminator >= 3:
		x_current = 0
		y_current += latice_spacing
	if deliminator >= 4:
		y_current = 0
		z_current += latice_spacing
	if deliminator >= 5:
		#print("5 deliminator!")
		frames.append(Frame(xE, yE, zE, uE, vE, wE, xB, yB, zB, uB, vB, wB))
		xE = []
		yE = []
		zE = []
		uE = []
		vE = [] 
		wE = []

		xB = []
		yB = []
		zB = []
		uB = []
		vB = []
		wB = []
		
	

	

fig = go.Figure()

for frame in frames:
	fig.add_trace(go.Cone(
		x=frame.xE,
		y=frame.yE,
		z=frame.zE,
		u=frame.uE,
		v=frame.vE,
		w=frame.wE,
		visible=False,
		name="Electric",
		colorscale="orrd",
		lighting_ambient=0.1, colorbar_x=0.92,
		sizemode="scaled",
		colorbar={"title":"Electric"}
	))

	fig.add_trace(go.Cone(
		x=frame.xB,
		y=frame.yB,
		z=frame.zB,
		u=frame.uB,
		v=frame.vB,
		w=frame.wB,
		visible=False,
		name="Magnetic",
		colorscale="viridis",
		lightposition=dict(x=0, y=0, z=1e5),
		sizemode="scaled",
		colorbar={"title":"Magnetic"}
	))

fig.data[0].visible = True
fig.data[1].visible = True

# Create and add slider
steps = []
for i in range(len(fig.data)//2):
	step = dict(
		method="update",
		args=[{"visible": [False] * len(fig.data)},
			  {"title": "Time: " + str(round(i * dt, 6))}],  # layout attribute
	)
	step["args"][0]["visible"][2*i] = True
	step["args"][0]["visible"][2*i + 1] = True
	steps.append(step)

sliders = [dict(
	active=0,
	currentvalue={"prefix": "Time: "},
	pad={"t": 50},
	steps=steps
)]

fig.update_layout(
	sliders=sliders
)
print("Data structured, displaying...")

fig.show()