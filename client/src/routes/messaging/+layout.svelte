<script>
    import { page } from '$app/stores';
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Button, Input,Avatar } from 'flowbite-svelte';
    $: activeUrl = $page.url.pathname;
    let activeClass = 'text-white bg-blue-700 md:bg-transparent md:text-blue-700 md:dark:text-white dark:bg-blue-600 md:dark:bg-transparent';
    let nonActiveClass = 'text-gray-700 hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent';
    import { SpeedDial, SpeedDialButton } from 'flowbite-svelte';
    import { ShareNodesSolid, SearchOutline, PhoneSolid, UserAddSolid, MessagesSolid } from 'flowbite-svelte-icons';
    import { Card, Listgroup } from 'flowbite-svelte';
    import { Indicator } from 'flowbite-svelte';
    
  
    let list = [
      {
        img: { src: '/images/profile-picture-1.webp', alt: 'Neil Sims' },
        name: 'Neil Sims',
        email: 'email@windster.com',
        phone: '123-456-7890'
      },
      {
        img: { src: '/images/profile-picture-2.webp', alt: 'Bonnie Green' },
        name: 'Bonnie Green',
        email: 'email@windster.com',
        phone: '123-456-7890'
      },
      {
        img: { src: '/images/profile-picture-3.webp', alt: 'Michael Gough' },
        name: 'Michael Gough',
        email: 'email@windster.com',
        phone: '123-456-7890'
      },
      {
        img: { src: '/images/profile-picture-3.webp', alt: 'Michael Gough' },
        name: 'Michael Gough',
        email: 'email@windster.com',
        phone: '123-456-7890'
      },
      {
        img: { src: '/images/profile-picture-3.webp', alt: 'Michael Gough' },
        name: 'Michael Gough',
        email: 'email@windster.com',
        phone: '123-456-7890'
      },
      {
        img: { src: '/images/profile-picture-3.webp', alt: 'Michael Gough' },
        name: 'Michael Gough',
        email: 'email@windster.com',
        phone: '123-456-7890'
      },
    ];

</script>
  

<div class="flex items-start justify-start h-screen">
  <Card padding="sm" size="xl">
    <div class="flex justify-between items-center mb-4">
      <h5 class="text-xl font-bold leading-none text-gray-900 dark:text-white">Contacts</h5>
      <a href="/messaging" class="text-sm font-medium text-blue-600 hover:underline dark:text-blue-500"> View all </a>
    </div>
    <Listgroup items={list} let:item class="border-0 dark:bg-transparent!">
      <div class="flex items-center space-x-4 rtl:space-x-reverse">
        <Avatar src="/images/profile-picture-5.webp" dot={{ color: 'green', size: 'lg', placement: 'top-right' }} />
        <!-- <Avatar src="/images/profile-picture-5.webp" dot={{ color: 'red', size: 'lg', placement: 'top-right' }} /> -->
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-gray-900 truncate dark:text-white">
            {item.name}
          </p>
          <p class="text-sm text-gray-500 truncate dark:text-gray-400">
            {item.email}
          </p>
        </div>
        <div class="inline-flex items-center text-base font-semibold text-gray-900 dark:text-white">
          {item.phone}
        </div>
      </div>
    </Listgroup>
  </Card>
</div>
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
    <SpeedDialButton name="Message">
        <MessagesSolid class="w-6 h-6" />
    </SpeedDialButton>
</SpeedDial>
