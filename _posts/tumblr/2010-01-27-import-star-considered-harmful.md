---
layout: post
title: Import * Considered Harmful
date: '2010-01-27T11:02:00-05:00'
tags:
- opinion
- java
- python
tumblr_url: https://seanmonstar.com/post/708954358/import-star-considered-harmful
---
Something a Java programmer learns first is that there is this big, amazing library already built-in to Java, and you can easily use plenty of useful classes by using an `import` statement. Possibly the first thing you want to do is pop open a box to prompt your name, or say hello, and thus starts this terrible habit:

    import javax.swing.*;

I’m guilty of it too. You don’t really know what you’re doing is all that bad. You know what you want from Swing. You only need the `JOptionPane`. And sure, the compiler shouldn’t be stupid enough to pack the rest of the Swing package into your jar file. In Java, at least, it won’t. There’s talk about whether certain bulk imports in Python will cause things to be included multiple times.

#### Collisions, or which List did you want?

However, in Java, you _can_get namespace collisions. [coobird on Stack Overflow gives an excellent example](http://stackoverflow.com/questions/1983435/eclipse-java-is-it-harmful-to-import-java-namespace/1983539#1983539):

> Now, if we were to use a wildcard in the package import, we’d have the following.
> 
> import java.awt.\*; import java.util.\*;
> 
> However, now we will have a problem!
> 
> There is a `java.awt.List` class and a `java.util.List`, so referring to the List class would be ambiguous. One would have to refer to the List with a fully-qualified class name if we want to remove the ambiguity:
> 
> import java.awt.\*; import java.util.\*;// 'List' from java.awt -- need to use a fully-qualified class name. java.awt.List listComponent = new java.awt.List()

This problem is exactly what I was trying to avoid when doing working with some Java, and prompted my need to let people know **never to do this** again. I was trying to call a class from the [YUI Compressor](http://developer.yahoo.com/yui/compressor/) jar, and the constructor required several classes. Unfamiliar with a couple of the names, I didn’t simply want to copy their import statements, since I already had written my own File class that was far more basic than Java’s. _No need for conflicts, please_.

Your code doesn’t have this problem? You’re only importing from one package, you say? What about the future of your code? Your class is still auto-importing the rest of your class’ residing package. What happens when someone creates a class called List? Or something else? _Conflicted_.

This leads to another frustrating reason not to use import star.

#### It screws Discoverability

Specifically, I was unsure which `ErrorReporter` was needed for the `JavaScriptCompressor`. The import statements at the top list 3 packages it could come from, and the only way for me to find out it to search each package.

    package com.yahoo.platform.yui.compressor; 
    import org.mozilla.javascript.*;
    import java.util.*;

ErrorReporter could be a class defined in this package (`com.yahoo.platform.yui.compressor`), or it could be `java.util`, or `org.mozilla.javascript`. It turns out it’s in the latter, but discovering that took longer than it ever should have. Even the few minutes I had to spend to lookup which package contained the class so I could import it into my class file was minutes wasted. It’s effortless to have to used a more specific import statement instead. Especially if you’re using an IDE like Eclipse (which you are if you’re doing Java development, just press Ctrl + Shift + O).

Flex Builder is an extension of Eclipse, so no excuses there either. I imagine Flash has a similar shortcut, though even if it didn’t, just like in Python, it’s really not that hard. Honestly, it takes **no extra effort** to write the name of a specific class instead of importing the whole dang package or module.

This reason is what I feel is the more important reason why **you shouldn’t use import \* ever again**. The more time it takes another programmer (or even yourself) to understand what in the world was going on inside your head at the time of writing, that’s time (and thus money) you’re costing your company.

