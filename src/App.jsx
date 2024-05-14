import { useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";

import { invoke } from "@tauri-apps/api";

function App() {
  const [count, setCount] = useState(0);

  // now we can call our Command!
  // Right-click the application background and open the developer tools.
  // You will see "Hello, World!" printed in the console!
  invoke("greet", { name: "World" })
    // `invoke` returns a Promise
    .then((response) => console.log(response));

  return (
    <>
      <h1 className="text-3xl font-bold underline">Hello world!</h1>
    </>
  );
}

export default App;
