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

	let profile = document.getElementById("profile");
	profile.value = inActionInfo.payload.settings.profile ?? "";

	document.getElementById("update").addEventListener("click", () => {
		websocket.send(JSON.stringify({
			event: "setSettings",
			context: inActionInfo.context,
			payload: {
				device: inActionInfo.device,
				profile: profile.value
			}
		}));
	});
}
