import React, { useState, useEffect } from "react";
import * as ReactDOM from "react-dom";
import init, { CounterApp } from "../suit-web/pkg/suit_web";

const App = ({ CounterApp }) => {
  const [app, setApp] = useState();
  const [counterState, setCounterState] = useState();

  useEffect(() => {
    const app = CounterApp.new((state) => setCounterState(state));
    setApp(app);
  }, []);

  return (
    <Counter
      state={counterState}
      handleDecrement={() => app.dispatch("dec")}
      handleIncrement={() => app.dispatch("inc")}
    />
  );
};

const Counter = ({ state, handleIncrement, handleDecrement }) => {
  const styles = {
    textAlign: "center",
    fontSize: "200%",
    fontFamily: "sans-serif",
  };

  return (
    <div style={styles}>
      <p>counter is: {state}</p>
      <button onClick={handleIncrement}>+</button>
      <button onClick={handleDecrement}>-</button>
    </div>
  );
};

init("suit-web/pkg/suit_web_bg.wasm").then(() => {
  ReactDOM.render(
    <App CounterApp={CounterApp} />,
    document.getElementById("app")
  );
});
