function connectElgatoStreamDeckSocket(inPort, inPropertyInspectorUUID, inRegisterEvent, inInfo, inActionInfo) {
	websocket = new WebSocket("ws://localhost:" + inPort);
	inInfo = JSON.parse(inInfo);
	inActionInfo = JSON.parse(inActionInfo);

	websocket.onopen = () => {
		websocket.send(JSON.stringify({
			event: inRegisterEvent,
			uuid: inPropertyInspectorUUID
		}));
	};

	let down = document.getElementById("down");
	down.value = inActionInfo.payload.settings.down ?? "";
	let up = document.getElementById("up");
	up.value = inActionInfo.payload.settings.up ?? "";

	document.getElementById("update").addEventListener("click", () => {
		websocket.send(JSON.stringify({
			event: "setSettings",
			context: inActionInfo.context,
			payload: {
				down: down.value,
				up: up.value
			}
		}));
	});
}
