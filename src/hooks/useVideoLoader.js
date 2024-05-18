import { invoke } from "@tauri-apps/api";

const useVideoLoader = (file) => {
  const createVideo = async () => {
    try {
      const response = await invoke("create_video_with_ffmpeg", { path: file.filepath });
      console.log(response);
    } catch (error) {
      console.error(error);
    }
  };

  return createVideo;
};

export default useVideoLoader;