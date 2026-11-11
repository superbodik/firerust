(function (global) {
  var natives = global.__fr;
  var nodeCache = new Map();
  var listenerStore = new WeakMap();
  var timers = new Map();
  var timerSequence = 0;
  var clock = 0;
  var readyState = 'loading';

  var BUBBLING_EVENTS = {
    click: true, dblclick: true, mousedown: true, mouseup: true, mousemove: true,
    mouseover: true, mouseout: true, keydown: true, keyup: true, keypress: true,
    input: true, change: true, submit: true, contextmenu: true, wheel: true
  };
  var CANCELABLE_EVENTS = {
    click: true, dblclick: true, mousedown: true, mouseup: true, keydown: true,
    keyup: true, keypress: true, submit: true, contextmenu: true, wheel: true
  };
  var HANDLER_EVENTS = [
    'click', 'dblclick', 'mousedown', 'mouseup', 'mousemove', 'mouseover', 'mouseout',
    'keydown', 'keyup', 'keypress', 'input', 'change', 'submit', 'focus', 'blur',
    'load', 'contextmenu', 'wheel'
  ];

  function describe(error) {
    if (error && error.message) {
      return (error.name ? error.name + ': ' : '') + error.message;
    }
    return String(error);
  }

  function reportError(error) {
    natives.log('error', 'Uncaught ' + describe(error));
  }

  function define(target, name, value) {
    Object.defineProperty(target, name, {
      value: value,
      writable: true,
      configurable: true,
      enumerable: false
    });
  }

  function listenersOf(target) {
    var table = listenerStore.get(target);
    if (!table) {
      table = {};
      listenerStore.set(target, table);
    }
    return table;
  }

  function normalizeOptions(options) {
    if (typeof options === 'boolean') {
      return { capture: options, once: false };
    }
    return {
      capture: !!(options && options.capture),
      once: !!(options && options.once)
    };
  }

  function addListener(target, type, callback, options) {
    if (!callback) {
      return;
    }
    var settings = normalizeOptions(options);
    var table = listenersOf(target);
    var list = table[type] || (table[type] = []);
    for (var i = 0; i < list.length; i++) {
      if (list[i].callback === callback && list[i].capture === settings.capture && !list[i].property) {
        return;
      }
    }
    list.push({ callback: callback, capture: settings.capture, once: settings.once, property: false });
  }

  function removeListener(target, type, callback, options) {
    var settings = normalizeOptions(options);
    var list = listenersOf(target)[type];
    if (!list) {
      return;
    }
    for (var i = 0; i < list.length; i++) {
      if (list[i].callback === callback && list[i].capture === settings.capture && !list[i].property) {
        list.splice(i, 1);
        return;
      }
    }
  }

  function setHandlerProperty(target, type, callback) {
    var table = listenersOf(target);
    var list = table[type] || (table[type] = []);
    for (var i = 0; i < list.length; i++) {
      if (list[i].property) {
        list.splice(i, 1);
        break;
      }
    }
    if (typeof callback === 'function') {
      list.push({ callback: callback, capture: false, once: false, property: true });
    }
  }

  function getHandlerProperty(target, type) {
    var list = listenersOf(target)[type] || [];
    for (var i = 0; i < list.length; i++) {
      if (list[i].property) {
        return list[i].callback;
      }
    }
    return null;
  }

  function parentOf(target) {
    if (target === global) {
      return null;
    }
    if (target === theDocument) {
      return global;
    }
    return target.parentNode;
  }

  var attributeHandlers = {};
  var inlineHandlersAllowed = true;

  function runAttributeHandler(target, event) {
    if (!target || target.nodeType !== 1) {
      return;
    }
    var code = inlineHandlersAllowed ? target.getAttribute('on' + event.type) : null;
    if (code === null) {
      return;
    }
    var key = target.__id + ':' + event.type;
    var cached = attributeHandlers[key];
    if (!cached || cached.code !== code) {
      try {
        cached = { code: code, run: new Function('event', code) };
      } catch (error) {
        reportError(error);
        return;
      }
      attributeHandlers[key] = cached;
    }
    event.currentTarget = target;
    try {
      if (cached.run.call(target, event) === false) {
        event.preventDefault();
      }
    } catch (error) {
      reportError(error);
    }
  }

  function invoke(target, event, phase) {
    if (event.__stopped) {
      return;
    }
    if (phase !== 'capture') {
      runAttributeHandler(target, event);
    }
    var list = listenersOf(target)[event.type];
    if (!list || list.length === 0) {
      return;
    }
    event.currentTarget = target;
    event.eventPhase = phase === 'capture' ? 1 : phase === 'target' ? 2 : 3;
    var snapshot = list.slice();
    for (var i = 0; i < snapshot.length; i++) {
      var entry = snapshot[i];
      if (event.__stoppedNow) {
        return;
      }
      if (phase === 'capture' && !entry.capture) {
        continue;
      }
      if (phase === 'bubble' && entry.capture) {
        continue;
      }
      if (entry.once) {
        removeListener(target, event.type, entry.callback, entry.capture);
      }
      try {
        if (typeof entry.callback === 'function') {
          entry.callback.call(target, event);
        } else if (entry.callback && typeof entry.callback.handleEvent === 'function') {
          entry.callback.handleEvent(event);
        }
      } catch (error) {
        reportError(error);
      }
    }
  }

  function dispatch(target, event) {
    define(event, 'target', target);
    var path = [];
    var current = target;
    while (current) {
      path.push(current);
      current = parentOf(current);
    }
    for (var i = path.length - 1; i > 0; i--) {
      invoke(path[i], event, 'capture');
    }
    invoke(path[0], event, 'target');
    if (event.bubbles) {
      for (var j = 1; j < path.length; j++) {
        invoke(path[j], event, 'bubble');
      }
    }
    event.currentTarget = null;
    event.eventPhase = 0;
    return !event.defaultPrevented;
  }

  class Event {
    constructor(type, init) {
      init = init || {};
      this.type = String(type);
      this.bubbles = !!init.bubbles;
      this.cancelable = !!init.cancelable;
      this.defaultPrevented = false;
      this.target = null;
      this.currentTarget = null;
      this.eventPhase = 0;
      this.isTrusted = false;
      this.timeStamp = clock;
      define(this, '__stopped', false);
      define(this, '__stoppedNow', false);
    }

    preventDefault() {
      if (this.cancelable) {
        this.defaultPrevented = true;
      }
    }

    stopPropagation() {
      this.__stopped = true;
    }

    stopImmediatePropagation() {
      this.__stopped = true;
      this.__stoppedNow = true;
    }
  }

  class CustomEvent extends Event {
    constructor(type, init) {
      super(type, init);
      this.detail = init && init.detail !== undefined ? init.detail : null;
    }
  }

  class UIEvent extends Event {}

  class MouseEvent extends UIEvent {
    constructor(type, init) {
      super(type, init);
      init = init || {};
      this.clientX = init.clientX || 0;
      this.clientY = init.clientY || 0;
      this.button = init.button || 0;
    }
  }

  class KeyboardEvent extends UIEvent {
    constructor(type, init) {
      super(type, init);
      init = init || {};
      this.key = init.key === undefined ? '' : String(init.key);
      this.code = init.code === undefined ? '' : String(init.code);
      this.ctrlKey = !!init.ctrlKey;
      this.shiftKey = !!init.shiftKey;
      this.altKey = !!init.altKey;
    }
  }

  function makeEvent(type, init) {
    init = init || {};
    var settings = {
      bubbles: init.bubbles !== undefined ? init.bubbles : !!BUBBLING_EVENTS[type],
      cancelable: init.cancelable !== undefined ? init.cancelable : !!CANCELABLE_EVENTS[type],
      clientX: init.clientX,
      clientY: init.clientY,
      button: init.button,
      key: init.key,
      code: init.code,
      ctrlKey: init.ctrlKey,
      shiftKey: init.shiftKey,
      altKey: init.altKey,
      detail: init.detail
    };
    if (type === 'click' || type === 'dblclick' || type.indexOf('mouse') === 0 || type === 'contextmenu') {
      return new MouseEvent(type, settings);
    }
    if (type.indexOf('key') === 0) {
      return new KeyboardEvent(type, settings);
    }
    return new Event(type, settings);
  }

  class EventTarget {
    addEventListener(type, callback, options) {
      addListener(this, String(type), callback, options);
    }

    removeEventListener(type, callback, options) {
      removeListener(this, String(type), callback, options);
    }

    dispatchEvent(event) {
      return dispatch(this, event);
    }
  }

  function wrap(id) {
    if (id < 0) {
      return null;
    }
    var cached = nodeCache.get(id);
    if (cached) {
      return cached;
    }
    var kind = natives.kind(id);
    var created;
    if (kind === 9) {
      created = new Document(id);
    } else if (kind === 1) {
      created = new HTMLElement(id);
    } else {
      created = new Text(id);
    }
    nodeCache.set(id, created);
    return created;
  }

  function idOf(node) {
    if (!node || typeof node.__id !== 'number') {
      throw new TypeError('The argument is not a Node');
    }
    return node.__id;
  }

  function wrapAll(ids) {
    var result = [];
    for (var i = 0; i < ids.length; i++) {
      result.push(wrap(ids[i]));
    }
    return result;
  }

  function kebab(name) {
    if (name === 'cssFloat') {
      return 'float';
    }
    return name.replace(/[A-Z]/g, function (letter) {
      return '-' + letter.toLowerCase();
    });
  }

  function readStyle(element) {
    var map = {};
    var order = [];
    var text = element.getAttribute('style') || '';
    var declarations = text.split(';');
    for (var i = 0; i < declarations.length; i++) {
      var index = declarations[i].indexOf(':');
      if (index > 0) {
        var name = declarations[i].slice(0, index).trim().toLowerCase();
        var value = declarations[i].slice(index + 1).trim();
        if (name) {
          if (!(name in map)) {
            order.push(name);
          }
          map[name] = value;
        }
      }
    }
    return { map: map, order: order };
  }

  function writeStyle(element, state) {
    var parts = [];
    for (var i = 0; i < state.order.length; i++) {
      var name = state.order[i];
      if (state.map[name] !== '') {
        parts.push(name + ': ' + state.map[name]);
      }
    }
    if (parts.length > 0) {
      element.setAttribute('style', parts.join('; '));
    } else {
      element.removeAttribute('style');
    }
  }

  function setStyleProperty(element, name, value) {
    var state = readStyle(element);
    var key = name.toLowerCase();
    if (!(key in state.map)) {
      state.order.push(key);
    }
    state.map[key] = String(value);
    writeStyle(element, state);
  }

  function makeStyle(element) {
    return new Proxy({}, {
      get: function (target, property) {
        if (typeof property !== 'string') {
          return undefined;
        }
        if (property === 'cssText') {
          return element.getAttribute('style') || '';
        }
        if (property === 'setProperty') {
          return function (name, value) {
            setStyleProperty(element, String(name), value);
          };
        }
        if (property === 'getPropertyValue') {
          return function (name) {
            return readStyle(element).map[String(name).toLowerCase()] || '';
          };
        }
        if (property === 'removeProperty') {
          return function (name) {
            var state = readStyle(element);
            var key = String(name).toLowerCase();
            var previous = state.map[key] || '';
            state.map[key] = '';
            writeStyle(element, state);
            return previous;
          };
        }
        return readStyle(element).map[kebab(property)] || '';
      },
      set: function (target, property, value) {
        if (typeof property !== 'string') {
          return true;
        }
        if (property === 'cssText') {
          element.setAttribute('style', String(value));
          return true;
        }
        setStyleProperty(element, kebab(property), value);
        return true;
      }
    });
  }

  class DOMTokenList {
    constructor(element) {
      define(this, '__element', element);
    }

    tokens() {
      var text = this.__element.getAttribute('class') || '';
      return text.split(/\s+/).filter(function (token) {
        return token.length > 0;
      });
    }

    save(tokens) {
      if (tokens.length > 0) {
        this.__element.setAttribute('class', tokens.join(' '));
      } else {
        this.__element.setAttribute('class', '');
      }
    }

    get length() {
      return this.tokens().length;
    }

    get value() {
      return this.__element.getAttribute('class') || '';
    }

    item(index) {
      var tokens = this.tokens();
      return index >= 0 && index < tokens.length ? tokens[index] : null;
    }

    contains(token) {
      return this.tokens().indexOf(String(token)) >= 0;
    }

    add() {
      var tokens = this.tokens();
      for (var i = 0; i < arguments.length; i++) {
        var token = String(arguments[i]);
        if (tokens.indexOf(token) < 0) {
          tokens.push(token);
        }
      }
      this.save(tokens);
    }

    remove() {
      var removed = Array.prototype.map.call(arguments, String);
      this.save(this.tokens().filter(function (token) {
        return removed.indexOf(token) < 0;
      }));
    }

    toggle(token, force) {
      token = String(token);
      var present = this.contains(token);
      var wanted = force === undefined ? !present : !!force;
      if (wanted && !present) {
        this.add(token);
      } else if (!wanted && present) {
        this.remove(token);
      }
      return wanted;
    }

    replace(oldToken, newToken) {
      var tokens = this.tokens();
      var index = tokens.indexOf(String(oldToken));
      if (index < 0) {
        return false;
      }
      tokens[index] = String(newToken);
      this.save(tokens);
      return true;
    }

    forEach(callback) {
      this.tokens().forEach(callback);
    }

    toString() {
      return this.value;
    }
  }

  function serialize(node) {
    var kind = node.nodeType;
    if (kind === 3) {
      return node.textContent.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }
    if (kind !== 1) {
      return '';
    }
    var name = node.localName;
    var text = '<' + name;
    var attributes = natives.attributes(node.__id);
    for (var i = 0; i < attributes.length; i += 2) {
      text += ' ' + attributes[i] + '="' + attributes[i + 1].replace(/&/g, '&amp;').replace(/"/g, '&quot;') + '"';
    }
    text += '>';
    if (name === 'br' || name === 'img' || name === 'hr' || name === 'input' || name === 'meta' || name === 'link') {
      return text;
    }
    return text + serializeChildren(node) + '</' + name + '>';
  }

  function serializeChildren(node) {
    var children = node.childNodes;
    var text = '';
    for (var i = 0; i < children.length; i++) {
      text += serialize(children[i]);
    }
    return text;
  }

  class Node extends EventTarget {
    constructor(id) {
      super();
      define(this, '__id', id);
    }

    get nodeType() {
      return natives.kind(this.__id);
    }

    get nodeName() {
      var kind = this.nodeType;
      if (kind === 1) {
        return natives.tagName(this.__id).toUpperCase();
      }
      return kind === 3 ? '#text' : '#document';
    }

    get parentNode() {
      return wrap(natives.parent(this.__id));
    }

    get parentElement() {
      var parent = this.parentNode;
      return parent && parent.nodeType === 1 ? parent : null;
    }

    get childNodes() {
      return wrapAll(natives.childNodes(this.__id));
    }

    get firstChild() {
      var children = this.childNodes;
      return children.length > 0 ? children[0] : null;
    }

    get lastChild() {
      var children = this.childNodes;
      return children.length > 0 ? children[children.length - 1] : null;
    }

    get nextSibling() {
      var parent = this.parentNode;
      if (!parent) {
        return null;
      }
      var siblings = parent.childNodes;
      var index = siblings.indexOf(this);
      return index >= 0 && index + 1 < siblings.length ? siblings[index + 1] : null;
    }

    get previousSibling() {
      var parent = this.parentNode;
      if (!parent) {
        return null;
      }
      var siblings = parent.childNodes;
      var index = siblings.indexOf(this);
      return index > 0 ? siblings[index - 1] : null;
    }

    get isConnected() {
      var current = this;
      while (current) {
        if (current.nodeType === 9) {
          return true;
        }
        current = current.parentNode;
      }
      return false;
    }

    get ownerDocument() {
      return theDocument;
    }

    get textContent() {
      return natives.textContent(this.__id);
    }

    set textContent(value) {
      natives.setTextContent(this.__id, value === null || value === undefined ? '' : String(value));
    }

    hasChildNodes() {
      return this.childNodes.length > 0;
    }

    contains(other) {
      while (other) {
        if (other === this) {
          return true;
        }
        other = other.parentNode;
      }
      return false;
    }

    appendChild(child) {
      natives.appendChild(this.__id, idOf(child));
      return child;
    }

    insertBefore(child, reference) {
      natives.insertBefore(this.__id, idOf(child), reference ? idOf(reference) : -1);
      return child;
    }

    removeChild(child) {
      if (natives.parent(idOf(child)) !== this.__id) {
        throw new Error('NotFoundError: The node to be removed is not a child of this node');
      }
      natives.removeChild(child.__id);
      return child;
    }

    replaceChild(replacement, existing) {
      this.insertBefore(replacement, existing);
      this.removeChild(existing);
      return existing;
    }

    cloneNode(deep) {
      var copy;
      if (this.nodeType === 3) {
        return wrap(natives.createText(this.textContent));
      }
      if (this.nodeType !== 1) {
        throw new Error('This node cannot be cloned');
      }
      copy = wrap(natives.createElement(natives.tagName(this.__id)));
      var attributes = natives.attributes(this.__id);
      for (var i = 0; i < attributes.length; i += 2) {
        copy.setAttribute(attributes[i], attributes[i + 1]);
      }
      if (deep) {
        var children = this.childNodes;
        for (var j = 0; j < children.length; j++) {
          copy.appendChild(children[j].cloneNode(true));
        }
      }
      return copy;
    }
  }

  HANDLER_EVENTS.forEach(function (type) {
    Object.defineProperty(Node.prototype, 'on' + type, {
      get: function () {
        return getHandlerProperty(this, type);
      },
      set: function (callback) {
        setHandlerProperty(this, type, callback);
      },
      configurable: true,
      enumerable: false
    });
  });

  class Text extends Node {
    get data() {
      return this.textContent;
    }

    set data(value) {
      this.textContent = value;
    }

    get nodeValue() {
      return this.textContent;
    }

    set nodeValue(value) {
      this.textContent = value;
    }

    get length() {
      return this.textContent.length;
    }
  }

  class Element extends Node {
    get tagName() {
      return natives.tagName(this.__id).toUpperCase();
    }

    get localName() {
      return natives.tagName(this.__id);
    }

    get id() {
      return this.getAttribute('id') || '';
    }

    set id(value) {
      this.setAttribute('id', value);
    }

    get className() {
      return this.getAttribute('class') || '';
    }

    set className(value) {
      this.setAttribute('class', value);
    }

    get classList() {
      if (!this.__classList) {
        define(this, '__classList', new DOMTokenList(this));
      }
      return this.__classList;
    }

    get style() {
      if (!this.__style) {
        define(this, '__style', makeStyle(this));
      }
      return this.__style;
    }

    get children() {
      return this.childNodes.filter(function (node) {
        return node.nodeType === 1;
      });
    }

    get childElementCount() {
      return this.children.length;
    }

    get firstElementChild() {
      var children = this.children;
      return children.length > 0 ? children[0] : null;
    }

    get lastElementChild() {
      var children = this.children;
      return children.length > 0 ? children[children.length - 1] : null;
    }

    get nextElementSibling() {
      var node = this.nextSibling;
      while (node && node.nodeType !== 1) {
        node = node.nextSibling;
      }
      return node;
    }

    get previousElementSibling() {
      var node = this.previousSibling;
      while (node && node.nodeType !== 1) {
        node = node.previousSibling;
      }
      return node;
    }

    get innerHTML() {
      return serializeChildren(this);
    }

    set innerHTML(value) {
      natives.setInnerHtml(this.__id, String(value));
    }

    get outerHTML() {
      return serialize(this);
    }

    get attributes() {
      var flat = natives.attributes(this.__id);
      var list = [];
      for (var i = 0; i < flat.length; i += 2) {
        list.push({ name: flat[i], value: flat[i + 1] });
      }
      return list;
    }

    get hidden() {
      return this.hasAttribute('hidden');
    }

    set hidden(value) {
      if (value) {
        this.setAttribute('hidden', '');
      } else {
        this.removeAttribute('hidden');
      }
    }

    getAttribute(name) {
      return natives.getAttribute(this.__id, String(name).toLowerCase());
    }

    setAttribute(name, value) {
      natives.setAttribute(this.__id, String(name).toLowerCase(), String(value));
    }

    removeAttribute(name) {
      natives.removeAttribute(this.__id, String(name).toLowerCase());
    }

    hasAttribute(name) {
      return this.getAttribute(name) !== null;
    }

    toggleAttribute(name, force) {
      var present = this.hasAttribute(name);
      var wanted = force === undefined ? !present : !!force;
      if (wanted && !present) {
        this.setAttribute(name, '');
      } else if (!wanted && present) {
        this.removeAttribute(name);
      }
      return wanted;
    }

    querySelectorAll(selector) {
      return wrapAll(natives.queryAll(this.__id, String(selector)));
    }

    querySelector(selector) {
      var found = this.querySelectorAll(selector);
      return found.length > 0 ? found[0] : null;
    }

    getElementsByTagName(name) {
      return this.querySelectorAll(String(name));
    }

    getElementsByClassName(names) {
      var selector = String(names).split(/\s+/).filter(function (name) {
        return name.length > 0;
      }).map(function (name) {
        return '.' + name;
      }).join('');
      return selector ? this.querySelectorAll(selector) : [];
    }

    matches(selector) {
      return natives.matches(this.__id, String(selector));
    }

    closest(selector) {
      var current = this;
      while (current && current.nodeType === 1) {
        if (current.matches(selector)) {
          return current;
        }
        current = current.parentNode;
      }
      return null;
    }

    remove() {
      natives.removeChild(this.__id);
    }

    append() {
      for (var i = 0; i < arguments.length; i++) {
        var item = arguments[i];
        this.appendChild(typeof item === 'object' && item !== null ? item : wrap(natives.createText(String(item))));
      }
    }

    prepend() {
      var first = this.firstChild;
      for (var i = 0; i < arguments.length; i++) {
        var item = arguments[i];
        this.insertBefore(typeof item === 'object' && item !== null ? item : wrap(natives.createText(String(item))), first);
      }
    }

    click() {
      dispatch(this, makeEvent('click', { clientX: 0, clientY: 0 }));
    }

    focus() {}

    blur() {}

    scrollIntoView() {}

    getBoundingClientRect() {
      return { x: 0, y: 0, top: 0, left: 0, right: 0, bottom: 0, width: 0, height: 0 };
    }
  }

  class HTMLElement extends Element {
    get value() {
      return this.getAttribute('value') || '';
    }

    set value(value) {
      this.setAttribute('value', value);
    }

    get checked() {
      return this.hasAttribute('checked');
    }

    set checked(value) {
      if (value) {
        this.setAttribute('checked', '');
      } else {
        this.removeAttribute('checked');
      }
    }

    get href() {
      return this.getAttribute('href') || '';
    }

    set href(value) {
      this.setAttribute('href', value);
    }

    get src() {
      return this.getAttribute('src') || '';
    }

    set src(value) {
      this.setAttribute('src', value);
    }

    get title() {
      return this.getAttribute('title') || '';
    }

    set title(value) {
      this.setAttribute('title', value);
    }

    get disabled() {
      return this.hasAttribute('disabled');
    }

    set disabled(value) {
      if (value) {
        this.setAttribute('disabled', '');
      } else {
        this.removeAttribute('disabled');
      }
    }
  }

  class Document extends Node {
    get documentElement() {
      var children = this.childNodes;
      for (var i = 0; i < children.length; i++) {
        if (children[i].nodeType === 1) {
          return children[i];
        }
      }
      return null;
    }

    get body() {
      return this.childOfRoot('body');
    }

    get head() {
      return this.childOfRoot('head');
    }

    childOfRoot(name) {
      var root = this.documentElement;
      if (!root) {
        return null;
      }
      var children = root.childNodes;
      for (var i = 0; i < children.length; i++) {
        if (children[i].nodeType === 1 && children[i].localName === name) {
          return children[i];
        }
      }
      return null;
    }

    get readyState() {
      return readyState;
    }

    get title() {
      var found = this.querySelector('title');
      return found ? found.textContent : '';
    }

    set title(value) {
      var found = this.querySelector('title');
      if (found) {
        found.textContent = value;
      }
    }

    get cookie() {
      return '';
    }

    set cookie(value) {}

    getElementById(id) {
      return wrap(natives.getElementById(String(id)));
    }

    createElement(name) {
      return wrap(natives.createElement(String(name).toLowerCase()));
    }

    createTextNode(text) {
      return wrap(natives.createText(String(text)));
    }

    createEvent(type) {
      return new Event(String(type));
    }

    querySelectorAll(selector) {
      return wrapAll(natives.queryAll(this.__id, String(selector)));
    }

    querySelector(selector) {
      var found = this.querySelectorAll(selector);
      return found.length > 0 ? found[0] : null;
    }

    getElementsByTagName(name) {
      return this.querySelectorAll(String(name));
    }

    getElementsByClassName(names) {
      return Element.prototype.getElementsByClassName.call(this, names);
    }
  }

  function makeStorage() {
    var items = {};
    return {
      getItem: function (key) {
        key = String(key);
        return Object.prototype.hasOwnProperty.call(items, key) ? items[key] : null;
      },
      setItem: function (key, value) {
        items[String(key)] = String(value);
      },
      removeItem: function (key) {
        delete items[String(key)];
      },
      clear: function () {
        items = {};
      },
      key: function (index) {
        var keys = Object.keys(items);
        return index >= 0 && index < keys.length ? keys[index] : null;
      },
      get length() {
        return Object.keys(items).length;
      }
    };
  }

  function addTimer(callback, delay, args, repeat) {
    timerSequence += 1;
    var wait = Math.max(0, Number(delay) || 0);
    timers.set(timerSequence, {
      id: timerSequence,
      callback: callback,
      args: args,
      due: clock + wait,
      interval: repeat ? Math.max(1, wait) : null
    });
    return timerSequence;
  }

  function clearTimer(id) {
    timers.delete(id);
  }

  function nextDueTimer() {
    var best = null;
    timers.forEach(function (timer) {
      if (!best || timer.due < best.due || (timer.due === best.due && timer.id < best.id)) {
        best = timer;
      }
    });
    return best;
  }

  function formatArguments(values) {
    var parts = [];
    for (var i = 0; i < values.length; i++) {
      var value = values[i];
      if (typeof value === 'string') {
        parts.push(value);
      } else if (value instanceof Error) {
        parts.push(describe(value));
      } else if (typeof value === 'object' && value !== null) {
        try {
          parts.push(JSON.stringify(value));
        } catch (error) {
          parts.push(String(value));
        }
      } else {
        parts.push(String(value));
      }
    }
    return parts.join(' ');
  }

  function levelLogger(level) {
    return function () {
      natives.log(level, formatArguments(arguments));
    };
  }

  var theDocument = wrap(0);

  global.window = global;
  global.self = global;
  global.globalThis = global;
  global.document = theDocument;
  global.Event = Event;
  global.CustomEvent = CustomEvent;
  global.UIEvent = UIEvent;
  global.MouseEvent = MouseEvent;
  global.KeyboardEvent = KeyboardEvent;
  global.EventTarget = EventTarget;
  global.Image = function (width, height) {
    var element = theDocument.createElement('img');
    if (width !== undefined) {
      element.setAttribute('width', String(width));
    }
    if (height !== undefined) {
      element.setAttribute('height', String(height));
    }
    return element;
  };
  function MessagePort() {
    this.onmessage = null;
    this.peer = null;
    this.listeners = [];
  }
  MessagePort.prototype.postMessage = function (data) {
    var peer = this.peer;
    if (!peer) {
      return;
    }
    setTimeout(function () {
      var event = { type: 'message', data: data, target: peer };
      if (typeof peer.onmessage === 'function') {
        peer.onmessage(event);
      }
      peer.listeners.slice().forEach(function (listener) {
        listener(event);
      });
    }, 0);
  };
  MessagePort.prototype.addEventListener = function (type, listener) {
    if (type === 'message') {
      this.listeners.push(listener);
    }
  };
  MessagePort.prototype.removeEventListener = function (type, listener) {
    this.listeners = this.listeners.filter(function (item) {
      return item !== listener;
    });
  };
  MessagePort.prototype.start = function () {};
  MessagePort.prototype.close = function () {
    this.peer = null;
  };
  global.MessagePort = MessagePort;
  global.MessageChannel = function () {
    this.port1 = new MessagePort();
    this.port2 = new MessagePort();
    this.port1.peer = this.port2;
    this.port2.peer = this.port1;
  };
  global.Node = Node;
  global.Element = Element;
  global.HTMLElement = HTMLElement;
  global.Text = Text;
  global.Document = Document;
  global.DOMTokenList = DOMTokenList;
  global.addEventListener = function (type, callback, options) {
    addListener(global, String(type), callback, options);
  };
  global.removeEventListener = function (type, callback, options) {
    removeListener(global, String(type), callback, options);
  };
  global.dispatchEvent = function (event) {
    return dispatch(global, event);
  };
  HANDLER_EVENTS.forEach(function (type) {
    Object.defineProperty(global, 'on' + type, {
      get: function () {
        return getHandlerProperty(global, type);
      },
      set: function (callback) {
        setHandlerProperty(global, type, callback);
      },
      configurable: true,
      enumerable: false
    });
  });

  global.console = {
    log: levelLogger('log'),
    info: levelLogger('info'),
    warn: levelLogger('warn'),
    error: levelLogger('error'),
    debug: levelLogger('debug')
  };
  global.alert = levelLogger('alert');
  global.confirm = function () {
    return false;
  };
  global.prompt = function () {
    return null;
  };
  global.navigator = { userAgent: 'firerust', language: 'en-US', platform: 'Windows' };
  global.localStorage = makeStorage();
  global.sessionStorage = makeStorage();
  global.performance = {
    now: function () {
      return clock;
    }
  };
  global.setTimeout = function (callback, delay) {
    return addTimer(callback, delay, Array.prototype.slice.call(arguments, 2), false);
  };
  global.setInterval = function (callback, delay) {
    return addTimer(callback, delay, Array.prototype.slice.call(arguments, 2), true);
  };
  global.clearTimeout = clearTimer;
  global.clearInterval = clearTimer;
  global.requestAnimationFrame = function (callback) {
    return addTimer(function () {
      callback(clock);
    }, 16, [], false);
  };
  global.cancelAnimationFrame = clearTimer;
  global.getComputedStyle = function (element) {
    return element.style;
  };
  global.matchMedia = function () {
    return { matches: false, addListener: function () {}, removeListener: function () {} };
  };
  global.queueMicrotask = function (callback) {
    Promise.resolve().then(callback);
  };

  var host = {
    init: function (address) {
      var href = String(address);
      var parts =
        /^([a-zA-Z][a-zA-Z0-9+.-]*:)(?:\/\/([^\/?#]*))?([^?#]*)(\?[^#]*)?(#.*)?$/.exec(href) || [];
      var protocol = parts[1] || '';
      var authority = parts[2] || '';
      var at = authority.lastIndexOf('@');
      var hostPart = at >= 0 ? authority.slice(at + 1) : authority;
      var colon = hostPart.lastIndexOf(':');
      var hasPort = colon >= 0 && hostPart.indexOf(']') < colon;
      var navigate = function (target, inNewTab) {
        natives.navigate(String(target), !!inNewTab);
      };
      var place = {
        protocol: protocol,
        host: hostPart,
        hostname: hasPort ? hostPart.slice(0, colon) : hostPart,
        port: hasPort ? hostPart.slice(colon + 1) : '',
        pathname: parts[3] || (authority ? '/' : ''),
        search: parts[4] || '',
        hash: parts[5] || '',
        origin: authority ? protocol + '//' + hostPart : 'null',
        toString: function () {
          return href;
        },
        reload: function () {
          navigate(href, false);
        },
        assign: function (target) {
          navigate(target, false);
        },
        replace: function (target) {
          navigate(target, false);
        }
      };
      Object.defineProperty(place, 'href', {
        enumerable: true,
        get: function () {
          return href;
        },
        set: function (target) {
          navigate(target, false);
        }
      });
      global.location = place;
      global.open = function (target) {
        navigate(target, true);
        return null;
      };
    },
    dispatch: function (id, type, init) {
      var target = wrap(id);
      if (!target) {
        return true;
      }
      return dispatch(target, makeEvent(String(type), init));
    },
    fire: function (type) {
      if (type === 'DOMContentLoaded') {
        readyState = 'interactive';
        dispatch(theDocument, new Event('DOMContentLoaded', { bubbles: true }));
      } else if (type === 'load') {
        readyState = 'complete';
        dispatch(global, new Event('load'));
      }
    },
    advance: function (now) {
      clock = Math.max(clock, Number(now) || 0);
      var guard = 0;
      for (;;) {
        var timer = nextDueTimer();
        if (!timer || timer.due > clock || guard > 10000) {
          break;
        }
        guard += 1;
        if (timer.interval !== null) {
          timer.due += timer.interval;
        } else {
          timers.delete(timer.id);
        }
        try {
          if (typeof timer.callback === 'function') {
            timer.callback.apply(global, timer.args);
          }
        } catch (error) {
          reportError(error);
        }
      }
    },
    nextTimer: function () {
      var timer = nextDueTimer();
      return timer ? timer.due : null;
    },
    forbidInlineHandlers: function () {
      inlineHandlersAllowed = false;
    }
  };
  define(global, '__frHost', host);
})(globalThis);
