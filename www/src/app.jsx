import React, { useState, useEffect } from "react";
import * as ReactDOM from "react-dom";
import init, { CounterApp, WebClient } from "../suit-web/pkg/suit_web";

const App = ({ CounterApp }) => {
  // holds the app "singleton"
  const [app, setApp] = useState();

  // used on the callback to update state of the view
  const [counterState, setCounterState] = useState();

  useEffect(() => {
    const app = CounterApp.new(
      WebClient.new({
        update: (state) => setCounterState(state),
        loadState: () => localStorage.getItem("counter_value"),
        saveState: (state) => localStorage.setItem("counter_value", state),
      })
    );
    setApp(app);
  }, []);

  return (
    <Counter
      state={counterState}
      handleDecrement={() => app.decrement()}
      handleIncrement={() => app.increment()}
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
