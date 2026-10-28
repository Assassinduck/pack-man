import { useState } from "react";
import { Button } from "./components/ui/button";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  return (
    <main className="container">
      <Button>Click me</Button>
    </main>
  );
}

export default App;
