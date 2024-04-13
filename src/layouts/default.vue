<template>
  <div class="min-h-0 flex-1 flex overflow-hidden">
    <nav
      aria-label="Sidebar"
      style="height: 100svh"
      class="hidden lg:block flex-shrink-0 bg-gray-800 overflow-y-auto"
    >
      <div
        style="width: 70px; height: 100svh"
        class="sidebar relative flex space-y-16 flex-col p-3 items-stretch"
      >
        <div class="grid grid-cols-1 justify-items-center gap-2">
          <button>
            <Icon name="heroicons:home" class="sidebar-icon block" />
          </button>
          <button>
            <Icon name="heroicons:squares-2x2" class="sidebar-icon block" />
          </button>
          <button>
            <Icon name="heroicons:server-stack" class="sidebar-icon block" />
          </button>
          <button>
            <Icon
              name="heroicons:information-circle"
              class="sidebar-icon block"
            />
          </button>
          <button class="place-self-end">
            <Icon
              name="heroicons:adjustments-horizontal"
              class="sidebar-icon block"
            />
          </button>
        </div>
      </div>
    </nav>
    <main class="main w-full p-4">
      <ScrollAreaRoot class="ScrollAreaRoot" style="--scrollbar-size: 10px">
        <ScrollAreaViewport class="ScrollAreaViewport">
            <slot/>
        </ScrollAreaViewPort>
        </ScrollAreaRoot>
    </main>
  </div>
</template>
<script></script>
<style style lang="scss">
@tailwind base;
@tailwind components;
@tailwind utilities;

$sidebar-text-color: #8a96a3;

.sidebar {
  background: #24292e;
  border-right: solid 1px #4a555f;
}

.main {
  background: #1f2428;
}

.sidebar-icon {
  width: 30px;
  height: 30px;
  color: $sidebar-text-color;
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
