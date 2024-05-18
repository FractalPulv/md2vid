import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

const useEventListeners = (setProgress, setStage) => {
    useEffect(() => {
      let unlistenProgress;
      let unlistenStage;
  
      listen('progress', (event) => {
        setProgress(event.payload);
      }).then(unlisten => { unlistenProgress = unlisten; });
  
      listen('stage', (event) => {
        setStage(event.payload);
      }).then(unlisten => { unlistenStage = unlisten; });
  
      return () => {
        unlistenProgress && unlistenProgress();
        unlistenStage && unlistenStage();
      };
    }, [setProgress, setStage]);
  };
  
  export default useEventListeners;