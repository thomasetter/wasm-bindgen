/* eslint-disable no-unused-vars */
import { chars } from './chars-list.js';
import * as pkg from './pkg';

let counters: pkg.Counter[] = [];
addCounter();
let b = document.getElementById('add-counter');
if (!b) throw new Error('Unable to find #add-counter');
b.addEventListener('click', ev => addCounter());

function addCounter() {
    let ctr = pkg.Counter.new(randomChar(), 0);
    counters.push(ctr);
    update();
}

function update() {
    let container = document.getElementById('container');
    if (!container) throw new Error('Unable to find #container in dom');
    while (container.lastElementChild) {
        if (container.lastElementChild.id == 'add-counter') break;
        container.removeChild(container.lastElementChild);
    }
    for (var i = 0; i < counters.length; i++) {
        let counter = counters[i];
        container.appendChild(newCounter(counter.key() + " stuff  aaat", counter.count(), (ev: MouseEvent) => {
            counter.increment();
            update();
        }));
    }
}

function randomChar() {
    console.log('randomChar');
    let idx = Math.floor(Math.random() * (chars.length - 1));
    console.log('index', idx);
    let ret = chars.splice(idx, 1)[0];
    console.log('char', ret);
    return ret;
}

function newCounter(key: string, value: number, cb: { (ev: MouseEvent): any }) {
    let container = document.createElement('div');
    container.setAttribute('class', 'counter');
    let title = document.createElement('h1');
    title.appendChild(document.createTextNode('Counter ' + key));
    container.appendChild(title);
    container.appendChild(newField('Count', value.toString()));
    let plus = document.createElement('button');
    plus.setAttribute('type', 'button');
    plus.setAttribute('class', 'plus-button');
    plus.appendChild(document.createTextNode('+'));
    plus.addEventListener('click', cb);
    container.appendChild(plus);
    return container;
}

function newField(key: string, value: string) {
    let ret = document.createElement('div');
    ret.setAttribute('class', 'field');
    let name = document.createElement('span');
    name.setAttribute('class', 'name');
    name.appendChild(document.createTextNode(key));
    ret.appendChild(name);
    let val = document.createElement('span');
    val.setAttribute('class', 'value');
    val.appendChild(document.createTextNode(value));
    ret.appendChild(val);
    return ret;
}
