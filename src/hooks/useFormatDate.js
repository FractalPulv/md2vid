export const formatDate = (filename, size) => {
    // Extract the date part of the filename and convert it to a Date object
    const dateString = filename.replace(".md", "");
    const date = new Date(dateString);
  
    // if size is long then weekday and month will be full name
    // if size is short then weekday and month will be short name
    const options = {
      weekday: size === "long" ? "long" : "short",
      day: "numeric",
      month: size === "long" ? "long" : "short",
      year: "numeric",
    };
  
    return date.toLocaleDateString(undefined, options);
  };
  
  const useFormatDate = (date) => {
    return formatDate(date, "long");
  };
  
  export default useFormatDate;