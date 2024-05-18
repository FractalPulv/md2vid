import { invoke } from "@tauri-apps/api";

const useFileContentLoader = (file) => {
  const getTextContent = async () => {
    console.log(file.filepath);
    if (!file || !file.filepath) {
      console.error('File or file path is not defined');
      return;
    }

    try {
      const response = await invoke("read_file_and_extract_frontmatter", { path: file.filepath });
      console.log(response);
    } catch (error) {
      console.error(error);
    }
  };

  return getTextContent;
};

export default useFileContentLoader;