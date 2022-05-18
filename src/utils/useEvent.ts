import { useEffect } from "preact/hooks";

export function useEvent<K extends keyof WindowEventMap>(type: K, listener: (this: Window, event: WindowEventMap[K]) => any) {
	useEffect(() => {
		addEventListener(type, listener);
		return () => removeEventListener(type, listener);
	}, []);
}