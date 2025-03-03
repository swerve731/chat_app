<script>
    import { page } from '$app/stores';
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Button, Input,Avatar } from 'flowbite-svelte';
    $: activeUrl = $page.url.pathname;
    let activeClass = 'text-white bg-blue-700 md:bg-transparent md:text-blue-700 md:dark:text-white dark:bg-blue-600 md:dark:bg-transparent';
    let nonActiveClass = 'text-gray-700 hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent';
    import { SpeedDial, SpeedDialButton } from 'flowbite-svelte';
    import { ShareNodesSolid, SearchOutline, PhoneSolid, UserAddSolid } from 'flowbite-svelte-icons';
  

</script>
  
  <Navbar>
    <NavBrand href="/">
      <img src="" class="me-3 h-6 sm:h-9" alt="image here" />
      <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">Message App</span>
    </NavBrand>
    <div class="flex md:order-1 object-right-top">
        <Avatar id="avatar-menu" src="/images/profile-picture-3.webp" />
    </div>
    <div class="flex md:order-2">
        <Button color="blue" data-collapse-toggle="mobile-menu-3" aria-controls="mobile-menu-3" aria-expanded="false" class="md:hidden text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-hidden focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 rounded-lg text-sm p-2.5 me-1">
          <SearchOutline color="blue" class="w-5 h-5" />
        </Button>
        <div class="flex object-right-top hidden relative md:block">
            <div class="flex absolute inset-y-0 start-0 items-center ps-3 pointer-events-none">
              <SearchOutline color="blue" class="w-4 h-4" />
            </div>
            <Input id="search-navbar" class="ps-10" placeholder="Search..." />
        </div>
    </div>
  </Navbar>

  <SpeedDial color="blue" defaultClass="absolute end-10 bottom-10">
    <SpeedDialButton name="Share">
      <ShareNodesSolid class="w-6 h-6" />
    </SpeedDialButton>
    <SpeedDialButton name="New Contact">
      <UserAddSolid class="w-6 h-6" />
    </SpeedDialButton>
    <SpeedDialButton name="Call">
      <PhoneSolid class="w-6 h-6" />
    </SpeedDialButton>
  </SpeedDial>

  <slot></slot>