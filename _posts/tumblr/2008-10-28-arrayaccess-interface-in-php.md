---
layout: post
title: ArrayAccess Interface in PHP
date: '2008-10-28T16:59:00-07:00'
tags:
- php
tumblr_url: https://seanmonstar.com/post/707046627/arrayaccess-interface-in-php
---
This past week, I was using PHP’s DOMDocument class to work with some XML generation. It’s pretty similar to Javascript’s DOM manipulation. The XML I was generating was a bunch of items with key attributes. () Learning how to extend a new class so that it behaves like a PHP Array made me _outright excited for PHP_ ! Let’s delve into the magic.

<figure class="tmblr-full" data-orig-height="150" data-orig-width="400"><img src="https://64.media.tumblr.com/73cab1a3de5eb734293d97486b0ba68f/b05e57841c0421ea-d4/s540x810/70c42b720d725876d77f4c738881e9dff7a0fd20.jpg" data-orig-height="150" data-orig-width="400"></figure>

I needed to create in XML:

    \<item key="object"\>DOMAIN\</item\>

I needed to create items in this nature for various different properties to communicate to a domain API. _Since this is essentially a hash, or associate array, it’d be awesome if it could work like one._ **Well, it can!**

    $dt\_assoc['object'] = 'DOMAIN';

After writing my class, that’s how I’m able to create XML items. It all lies in using an object that implements the `ArrayAccess` interface. Doing so gives the object functions that are called when you try to access the object like an array. So by implementing the functions, you can write custom logic that happens, besides just simply holding a value like normal arrays do.

The above statement calls [ArrayAccess::offsetSet($offset,$value);](http://www.php.net/~helly/php/ext/spl/interfaceArrayAccess.html#_details) , so here’s the implementation.

    public function offsetSet($offset,$value) { $item = $this-\>dom-\>createElement('item',$value); $attr = $this-\>dom-\>createAttribute('key'); $item-\>appendChild($attr); $text = $this-\>dom-\>createTextNode($offset); $attr-\>appendChild($text); $this-\>items[$offset] = $item; $this-\>el-\>appendChild($item); }

Using DOMDocument to create XML for me, I create an item wrapped around the value, create an attribute called key, and give text of the offset. So instead of needing to write this process out for every new item, I just need to set a new key and value of an object that implements this interface.

There are 3 other methods to the interface, as well as one more function I define to export the data of this object into XML.

The constructor takes a name of an element to wrap all the items in, as well as the instance of the DOMDocument. You need to hold onto the original instance. This class creates an XML element of `<$name>items go here</$name>` .

The array `$data` is an associate array in the object to keep track of all the values of every item. The array items keeps a reference to the item nodes in the DOMDocument, so they can be easily removed with _offsetUnset()_ .

offsetExists simply checks if there is an item with the key of offset, and offsetGet returns the value of the item with the appropriate key.

Lastly, toXML() is a new function unrelated to ArrayAccess, but useful to me, which just returns the DOMElement that is holding onto all these items.

Here’s the full class. You’ll find [DOMDocument](http://php.net/domdocument) and [ArrayAccess](http://www.php.net/~helly/php/ext/spl/interfaceArrayAccess.html#_details) useful as well, if you need to do something similar.

    class SRSData implements ArrayAccess { var $el; var $dom; var $data; var $items; public function \_\_construct($name,$dom) { $this-\>el = $this-\>dom-\>createElement($name); } public function offsetExists($offset) { return isset($this-\>items[$offset]); } public function offsetGet($offset) { return $this-\>data[$offset]; } public function offsetSet($offset,$value) { if($value instanceof SRSData) { $item = $this-\>dom-\>createElement('item'); $item-\>appendChild($value-\>toXML()); } else { if(!empty($value)) $item = $this-\>dom-\>createElement('item',$value); else $item = $this-\>dom-\>createElement('item'); } $this-\>data[$offset] = $value; $attr = $this-\>dom-\>createAttribute('key'); $item-\>appendChild($attr); $text = $this-\>dom-\>createTextNode($offset); $attr-\>appendChild($text); $this-\>items[$offset] = $item; $this-\>el-\>appendChild($item); } public function offsetUnset($offset) { $this-\>el-\>removeChild($this-\>items[$offset]); unset($this-\>data[$offset]); unset($this-\>items[$offset]); } public function toXML() { return $this-\>el; } }

Learning this was one of those time where I was actually gleeful about programming. Hopefully, this helps show how powerful PHP really is.

