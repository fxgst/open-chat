<script lang="ts">
    import type { OpenChat } from "openchat-client";
    import { getContext } from "svelte";
    import Translatable from "../Translatable.svelte";
    import { i18nKey } from "../../i18n/i18n";
    import Overlay from "../Overlay.svelte";
    import ModalContent from "../ModalContent.svelte";
    import ButtonGroup from "../ButtonGroup.svelte";
    import Button from "../Button.svelte";

    const client = getContext<OpenChat>("client");

    $: notificationStatus = client.notificationStatus;
    $: anonUser = client.anonUser;

    $: console.debug("PUSH STATUS: ", $notificationStatus);
</script>

{#if !$anonUser && $notificationStatus === "prompt"}
    <Overlay dismissible>
        <ModalContent>
            <div class="header" slot="header">
                <Translatable resourceKey={i18nKey("Notifications")} />
            </div>
            <div class="body" slot="body">
                <Translatable resourceKey={i18nKey("enableNotifications")} />
            </div>
            <span slot="footer">
                <ButtonGroup>
                    <Button on:click={() => client.askForNotificationPermission()}
                        ><Translatable resourceKey={i18nKey("yesPlease")} /></Button>
                    <Button secondary on:click={() => client.setSoftDisabled(true)}
                        ><Translatable resourceKey={i18nKey("noThanks")} /></Button>
                </ButtonGroup>
            </span>
        </ModalContent>
    </Overlay>
{/if}
