<script>
  import { Card, Listgroup, Avatar } from 'flowbite-svelte';
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
<div class="flex items-center justify-center h-screen">
<Card padding="xl" size="md">
  <div class="flex justify-between items-center mb-4">
    <h5 class="text-xl font-bold leading-none text-gray-900 dark:text-white">Contacts</h5>
    <a href="/" class="text-sm font-medium text-blue-600 hover:underline dark:text-blue-500"> View all </a>
  </div>
  <Listgroup items={list} let:item class="border-0 dark:bg-transparent!">
    <div class="flex items-center space-x-4 rtl:space-x-reverse">
      <Avatar src={item.img.src} alt={item.img.alt} class="shrink-0" />
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