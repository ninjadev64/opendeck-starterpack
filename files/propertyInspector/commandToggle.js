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

	let down = document.getElementById("S1Down");
	down.value = inActionInfo.payload.settings.down ?? "";
	let up = document.getElementById("S1Up");
	up.value = inActionInfo.payload.settings.up ?? "";
	let down2 = document.getElementById("S2Down");
	down2.value = inActionInfo.payload.settings.down2 ?? "";
	let up2 = document.getElementById("S2Up");
	up2.value = inActionInfo.payload.settings.up2 ?? "";

	document.getElementById("update").addEventListener("click", () => {
		websocket.send(JSON.stringify({
			event: "setSettings",
			context: inActionInfo.context,
			payload: {
				down: down.value,
				up: up.value,
				down2: down2.value,
				up2: up2.value
			}
		}));
	});
}
