<template>
<div class="sidebar">
    <div class="grid grid-cols-1 justify-items-center gap-5">
        <button class="sidebar-icon-btn">
        <Icon name="heroicons:home" class="sidebar-icon block" />
        </button>
        <button class="sidebar-icon-btn">
        <Icon name="heroicons:squares-2x2" class="sidebar-icon block" />
        </button>
        <button class="sidebar-icon-btn">
        <Icon name="heroicons:server-stack" class="sidebar-icon block" />
        </button class="sidebar-icon-btn">
        <span class="h-full"></span>
        <button class="sidebar-icon-btn absolute bottom-16">
        <Icon
            name="heroicons:information-circle"
            class="sidebar-icon block"
        />
        </button>
        <button class="sidebar-icon-btn absolute bottom-5">
        <Icon
            name="heroicons:adjustments-horizontal"
            class="sidebar-icon block"
        />
        </button>
    </div>
</div>

<div class="main">
    <slot />
</div>

</template>
<script></script>
<style style lang="scss">
@tailwind base;
@tailwind components;
@tailwind utilities;

$main-bg: #1f2428;
$sidebar-txt-color: #8a96a3;

body {
  font-family: "Lato", sans-serif;
}

.sidebar {
  height: 100%;
  width: 58px;
  position: fixed;
  z-index: 1;
  top: 0;
  left: 0;
  overflow-x: hidden;
  padding-top: 20px;
  background-color: #24292e;
  border-right: solid 1px #4a555f;
}


.sidebar-icon {
  width: 30px;
  height: 30px;
}

.sidebar-icon-btn{
  color: $sidebar-txt-color;
}

.sidebar-icon-btn:hover{
  color: white;
}

.sidebar a:hover {
  color: #f1f1f1;
}

.main {
  margin-left: 58px; /* 58px for sidebar */
  font-size: 28px; /* Increased text to enable scrolling */
  padding-left: 40px;
  padding-right: 40px;
  padding-top: 20px;
  padding-bottom: 0;
  background: $main-bg;
}

body{
  background: $main-bg;
}

@media screen and (max-height: 450px) {
  .sidebar {padding-top: 15px;}
  .sidebar a {font-size: 18px;}
}

/*
  1. Use a more-intuitive box-sizing model.
*/
*,
*::before,
*::after {
  box-sizing: border-box;
}
/*
    2. Remove default margin
  */
* {
  margin: 0;
}
/*
    Typographic tweaks!
    3. Add accessible line-height
    4. Improve text rendering
  */
body {
  line-height: 1.5;
  -webkit-font-smoothing: antialiased;
}
/*
    5. Improve media defaults
  */
img,
picture,
video,
canvas,
svg {
  display: block;
  max-width: 100%;
}
/*
    6. Remove built-in form typography styles
  */
input,
button,
textarea,
select {
  font: inherit;
}
/*
    7. Avoid text overflows
  */
p,
h1,
h2,
h3,
h4,
h5,
h6 {
  overflow-wrap: break-word;
}
/*
    8. Create a root stacking context
  */
#root,
#__next {
  isolation: isolate;
}
</style>
