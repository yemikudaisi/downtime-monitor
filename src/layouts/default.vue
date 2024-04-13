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