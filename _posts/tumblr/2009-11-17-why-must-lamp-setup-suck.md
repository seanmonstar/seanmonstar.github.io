---
layout: post
title: Why Must LAMP Setup Suck?
date: '2009-11-17T15:15:00-05:00'
tags:
- lamp
- opinion
tumblr_url: https://seanmonstar.com/post/708843270/why-must-lamp-setup-suck
---
LAMP is common lingo for web developers. It’s an incredibly popular software stack to run dynamic websites. Many hosting companies include the stack for you, already configured and ready to go. But before we get there? That wonderful point where we trust our code to the production server, and then watch something blow up beautifully in a fire-y mess of status codes, fatal errors, and SQLStates. We developers like to test thing before then.

So I’m left wondering, **why does setting up a testing environment suck so hard**? Downloading a virtual machine is easy enough, and then you must download your flavor \*nix. Boot it up, apt-get your flavor’s Apache, MySQL, and PHP. Getting Apache to work just the way you want, making sure you set up the right hosts, and run the correct modules.

While you’re at it, your apt-get listscould be wrong, or not contain all the files you want. And in the middle of one installation, you may find you hit your memory limit, that simply requires increasing to a supreme value.

There are competent, [1-button install programs](http://www.apachefriends.org/en/xampp-windows.html) that setup a [WAMP](http://en.wikipedia.org/wiki/WAMP) environment. No hassles. Of course, the only downside being that the environment differs slightly than your production environment.

I recognize anyone needing a Linux server mostly knows how to set one up. But at the same time, the people wanting Linux servers are programmers. Isn’t our job _obsession_ all about automating tasks that can be automated, and forever trying to make every task in that category? Certainly, it must be a common setup to have a Debian (or Ubuntu, or RedHat, etc) Virtual Machine using Apache, SQL, and PHP. Even desiring Python to be installed, with mod\_python or mod\_wsgi. Or how about coming with Ruby set up for you. These would all be excellent downloadable Virtual Machines that programmers would use regularly.

#### Only One Solution So Far

I’ve seen one installer for a Linux virtual machine, though not in the flavor I regularly use: [SUSE Studio](http://susestudio.com/).

> Build an appliance - or your own custom Linux distro - with a few mouse clicks. Customize it to your heart’s content, and share it with the world.

The concept is brilliant. If you want a SUSE virtual machine, you just login, select the version, check some packages to include, such as Apache, MySQL, and the like. And then it builds it for you. Check back in a couple minutes, and you can download it to your machine. Or you can start it up right there in the web browser.

<figure class="tmblr-full" data-orig-height="500" data-orig-width="500"><img src="https://64.media.tumblr.com/cb298e3393f2b8a6eea9ab8a4da15c64/286dd1b45e2b4148-9a/s540x810/6601d584d56b12c5790199b449f5d9374f081ec3.png" data-orig-height="500" data-orig-width="500"></figure>

If I were the technical director at a company, and we were going to be using a new development area for a new project, I could easily customize a box (with clicking, not with tedious command line memorization), and after it’s built, give it out to all the developers. That’d be huge! It certainly is a pain point to make sure all us developers have a virtual machine with the same settings.

#### The Way Forward

SUSE Studio has the right idea, for all things that should be installed. Anything that requires a specific set of instructions that you’re requiring humans to do, every single time, should likely be programmed instead. Hopefully we’ll see other versions showing up, so the PHP developer wanting to check out Ruby, for example, can just download a virtual machine with it already set up, and get to the important stuff: **writing code and solve problems**.

