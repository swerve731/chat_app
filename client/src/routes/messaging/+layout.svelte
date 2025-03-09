<script>
  import { page } from '$app/stores';
  import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Button, Input,Avatar } from 'flowbite-svelte';
  $: activeUrl = $page.url.pathname;
  let activeClass = 'text-white bg-blue-700 md:bg-transparent md:text-blue-700 md:dark:text-white dark:bg-blue-600 md:dark:bg-transparent';
  let nonActiveClass = 'text-gray-700 hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent';
  import { SpeedDial, SpeedDialButton } from 'flowbite-svelte';
  import { ShareNodesSolid, SearchOutline, PhoneSolid, UserAddSolid, UserSettingsSolid  } from 'flowbite-svelte-icons';


</script>

<Navbar>
  <NavBrand href="/" class = "flex items-center gap-2">
    <img src="/images/logo.svg" alt="logo" class="w-10 h-10" />
    <span class="self-center whitespace-nowrap text-3xl font-semibold dark:text-white">Messages</span>
  </NavBrand>
    <div class="flex items-center gap-4">
    <Avatar  />
    <button class="p-2 rounded-full bg-blue-700 text-white dark:bg-blue-600 dark:text-white">
      <UserSettingsSolid class="w-6 h-6" />
    </button>
  </div>
</Navbar>


<slot></slot>