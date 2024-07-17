from fastapi import FastAPI
from fastapi.websockets import WebSocket, WebSocketDisconnect
import json

app = FastAPI()

class ConnectionManager:
	def __init__(self):
		self.active_connections: list[WebSocket] = []

	async def connect(self, ws: WebSocket):
		await ws.accept()
		self.active_connections.append(ws)

	def disconnect(self, ws: WebSocket):
		self.active_connections.remove(ws)

	async def broadcast(self, message: str):
		for connection in self.active_connections:
			await connection.send_text(message)

manager = ConnectionManager()

state = {}

@app.websocket("/media/nowplaying/{user}/ws")
async def read(ws: WebSocket, user: str):
	if user not in state:
		state[user] = {}
	await manager.connect(ws)
	# print("Client connected")
	try:
		while True:
			r = await ws.receive_text()
			if r == 'get':
				await ws.send_text(json.dumps(state[user]))
				continue

			data = json.loads(r)

			if 'now_playing' in data:
				state[user]['now_playing'] = data['now_playing']

			if 'position_modified' in data:
				state[user]['position_modified'] = data['position_modified']

			if 'position' in data:
				state[user]['position'] = data['position']

			if 'clear' in data:
				if data['clear']:
					state[user] = {}
			
			print(state)

			await manager.broadcast(json.dumps(state[user]))

			print(data)
	except WebSocketDisconnect:
		# print("Client disconnected")
		manager.disconnect(ws)