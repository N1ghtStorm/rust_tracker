import React from 'react';
import logo from './logo.svg';
import './App.css';
import {v4 as uuidv4} from 'uuid';
import events from 'events';


function App() {


  return (
    <div className="App">
      <button>PushButton</button>
      <button>PullButton</button>
      <button>DublicateButton</button>
    </div>
  );
}

let onPushButtonClick = () => {
  let event: Event = {
    uuid: uuidv4(),
    action: "Push",
    date: new Date(),
    buttonName:"Push button"
  } 
}

let onPullButtonClick = () => {
  let event: Event = {
    uuid: uuidv4(),
    action: "Pull",
    date: new Date(),
    buttonName:"Pull button"
  } 
}

let onDublicateButtonClick = () => {
  let u = uuidv4();
  let a: EventBatch = {
    events: [
      {
        uuid: u,
        action: "Dub",
        date: new Date(),
        buttonName:"Dub button 1"
      },
      {
        uuid: u,
        action: "Dub",
        date: new Date(),
        buttonName:"Dub button 2"
      }
    ]
  }
}

type Event = {
  uuid: string,
  action: string,
  date: Date,
  buttonName: string,
}

type EventBatch = {
  events: Event[]
}

export default App;
