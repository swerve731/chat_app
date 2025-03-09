<script>
    import { Label, Input, Button } from 'flowbite-svelte';
    import { RocketSolid } from 'flowbite-svelte-icons';

    let message = ""; // Store the input message
    let messages = /** @type {{ text: string, sentByUser: boolean }[]} */ ([]); // Store all sent messages

    function sendMessage() {
        if (message.trim() !== "") {
            messages = [...messages, { text: message, sentByUser: true }];
            message = ""; // Clear input after sending
        }
    }

    function handleKeyPress(event) {
        if (event.key === "Enter") {
            sendMessage();
        }
    }
</script>



<div class="flex flex-col h-screen bg-white-100 dark:bg-white-900">
    <div class="flex-1 overflow-y-auto p-4 space-y-2 bg-white dark:bg-gray-800 shadow-md rounded-lg mx-4 mt-4">
        {#each messages as msg}
            <div class="flex {msg.sentByUser ? 'justify-end' : 'justify-start'}">
                <div class="px-4 py-2 rounded-lg text-white max-w-[75%]"
                     class:sentByUser={"bg-blue-500"}
                     class:notSentByUser={"bg-gray-700"}>
                    {msg.text}
                </div>
            </div>
        {/each}
    </div>

   
    <div class="p-4 bg-white dark:bg-gray-800 shadow-lg flex items-center gap-2 sticky bottom-0">
        <Input
            id="default-input"
            placeholder="Type a message..."
            bind:value={message}
            on:keypress={handleKeyPress}
            class="flex-1 rounded-full h-14 py-3 px-4 text-lg border border-gray-300 dark:border-gray-600"
        />
        <Button color="blue" on:click={sendMessage} class="p-3 rounded-full h-14 w-14 flex items-center justify-center">
            <RocketSolid class="w-8 h-8" />
        </Button>
    </div>
</div>