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

	let s1_down = document.getElementById("s1down");
	s1_down.value = inActionInfo.payload.settings.down ?? "";
	let s1_up = document.getElementById("s1up");
	s1_up.value = inActionInfo.payload.settings.up ?? "";
	let s2_down = document.getElementById("s2down");
	s2_down.value = inActionInfo.payload.settings.down2 ?? "";
	let s2_up = document.getElementById("s2up");
	s2_up.value = inActionInfo.payload.settings.up2 ?? "";

	document.getElementById("update").addEventListener("click", () => {
		websocket.send(JSON.stringify({
			event: "setSettings",
			context: inActionInfo.context,
			payload: {
				s1_down: s1_down.value,
				s1_up: s1_up.value,
				s2_down: s2_down.value,
				s2_up: s2_up.value
			}
		}));
	});
}
