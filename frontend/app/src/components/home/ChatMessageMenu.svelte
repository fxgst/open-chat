<script lang="ts">
    import Menu from "../Menu.svelte";
    import MenuItem from "../MenuItem.svelte";
    import MenuIcon from "../MenuIcon.svelte";
    import ChevronDown from "svelte-material-icons/ChevronDown.svelte";
    import PencilOutline from "svelte-material-icons/PencilOutline.svelte";
    import ContentCopy from "svelte-material-icons/ContentCopy.svelte";
    import Reply from "svelte-material-icons/Reply.svelte";
    import Cancel from "svelte-material-icons/Cancel.svelte";
    import ReplyOutline from "svelte-material-icons/ReplyOutline.svelte";
    import DeleteOutline from "svelte-material-icons/DeleteOutline.svelte";
    import DeleteOffOutline from "svelte-material-icons/DeleteOffOutline.svelte";
    import TranslateIcon from "svelte-material-icons/Translate.svelte";
    import TranslateOff from "svelte-material-icons/TranslateOff.svelte";
    import ForwardIcon from "svelte-material-icons/Share.svelte";
    import Pin from "svelte-material-icons/Pin.svelte";
    import PinOff from "svelte-material-icons/PinOff.svelte";
    import ShareIcon from "svelte-material-icons/ShareVariant.svelte";
    import EyeOff from "svelte-material-icons/EyeOff.svelte";
    import HoverIcon from "../HoverIcon.svelte";
    import { _, locale } from "svelte-i18n";
    import { translationCodes } from "../../i18n/i18n";
    import { rtlStore } from "../../stores/rtl";
    import { iconSize } from "../../stores/iconSize";
    import { createEventDispatcher, getContext } from "svelte";
    import type { Message, OpenChat } from "openchat-client";
    import { push } from "svelte-spa-router";
    import { toastStore } from "../../stores/toast";

    const dispatch = createEventDispatcher();
    const client = getContext<OpenChat>("client");

    export let chatId: string;
    export let senderId: string;
    export let isProposal: boolean;
    export let inert: boolean;
    export let publicGroup: boolean;
    export let confirmed: boolean;
    export let canShare: boolean;
    export let me: boolean;
    export let canPin: boolean;
    export let pinned: boolean;
    export let supportsReply: boolean;
    export let canQuoteReply: boolean;
    export let inThread: boolean;
    export let canStartThread: boolean;
    export let groupChat: boolean;
    export let canForward: boolean;
    export let canBlockUser: boolean;
    export let canEdit: boolean;
    export let canDelete: boolean;
    export let canUndelete: boolean;
    export let translatable: boolean;
    export let translated: boolean;
    export let crypto: boolean;
    export let msg: Message;

    $: translationStore = client.translationStore;
    $: storageStore = client.storageStore;

    function blockUser() {
        dispatch("blockUser", { userId: senderId });
    }

    function collapseMessage() {
        dispatch("collapseMessage");
    }

    function shareMessage() {
        dispatch("shareMessage", msg);
    }

    function copyMessageUrl() {
        dispatch("copyMessageUrl", msg);
    }

    function pinMessage() {
        dispatch("pinMessage", msg);
    }

    function unpinMessage() {
        dispatch("unpinMessage", msg);
    }

    // this is called if we are starting a new thread so we pass undefined as the threadSummary param
    function initiateThread() {
        if (msg.thread !== undefined) {
            push(`/${chatId}/${msg.messageIndex}`);
        } else {
            client.openThread(msg.messageId, msg.messageIndex, true);
        }
    }

    function forward() {
        dispatch("forward", msg);
    }

    function deleteMessage() {
        dispatch("deleteMessage", msg);
    }

    function undeleteMessage() {
        dispatch("undeleteMessage", msg);
    }

    function untranslateMessage() {
        translationStore.untranslate(msg.messageId);
    }

    function translateMessage() {
        if ($storageStore.byteLimit === 0) {
            dispatch("upgrade", "premium");
        } else {
            if (msg.content.kind === "text_content") {
                const params = new URLSearchParams();
                params.append("q", msg.content.text);
                params.append("target", translationCodes[$locale || "en"] || "en");
                params.append("format", "text");
                params.append("key", process.env.PUBLIC_TRANSLATE_API_KEY!);
                fetch(`https://translation.googleapis.com/language/translate/v2?${params}`, {
                    method: "POST",
                })
                    .then((resp) => resp.json())
                    .then(({ data: { translations } }) => {
                        if (
                            msg.content.kind === "text_content" &&
                            Array.isArray(translations) &&
                            translations.length > 0
                        ) {
                            translationStore.translate(
                                msg.messageId,
                                translations[0].translatedText
                            );
                        }
                    })
                    .catch((_err) => {
                        toastStore.showFailureToast("unableToTranslate");
                    });
            }
        }
    }
</script>

<div class="menu" class:rtl={$rtlStore}>
    <MenuIcon centered>
        <div class="menu-icon" slot="icon">
            <HoverIcon compact={true}>
                <ChevronDown size="1.6em" color={me ? "#fff" : "var(--icon-txt)"} />
            </HoverIcon>
        </div>
        <div slot="menu">
            <Menu centered>
                {#if isProposal && !inert}
                    <MenuItem on:click={collapseMessage}>
                        <EyeOff size={$iconSize} color={"var(--icon-inverted-txt)"} slot="icon" />
                        <div slot="text">{$_("proposal.collapse")}</div>
                    </MenuItem>
                {/if}
                {#if publicGroup && confirmed && !inert}
                    {#if canShare}
                        <MenuItem on:click={shareMessage}>
                            <ShareIcon
                                size={$iconSize}
                                color={"var(--icon-inverted-txt)"}
                                slot="icon" />
                            <div slot="text">{$_("share")}</div>
                        </MenuItem>
                    {/if}
                    <MenuItem on:click={copyMessageUrl}>
                        <ContentCopy
                            size={$iconSize}
                            color={"var(--icon-inverted-txt)"}
                            slot="icon" />
                        <div slot="text">{$_("copyMessageUrl")}</div>
                    </MenuItem>
                {/if}
                {#if confirmed && canPin && !inThread && !inert}
                    {#if pinned}
                        <MenuItem on:click={unpinMessage}>
                            <PinOff
                                size={$iconSize}
                                color={"var(--icon-inverted-txt)"}
                                slot="icon" />
                            <div slot="text">{$_("unpinMessage")}</div>
                        </MenuItem>
                    {:else}
                        <MenuItem on:click={pinMessage}>
                            <Pin size={$iconSize} color={"var(--icon-inverted-txt)"} slot="icon" />
                            <div slot="text">{$_("pinMessage")}</div>
                        </MenuItem>
                    {/if}
                {/if}
                {#if confirmed && supportsReply && !inert}
                    {#if canQuoteReply}
                        <MenuItem on:click={() => dispatch("reply")}>
                            <Reply
                                size={$iconSize}
                                color={"var(--icon-inverted-txt)"}
                                slot="icon" />
                            <div slot="text">{$_("quoteReply")}</div>
                        </MenuItem>
                    {/if}
                    {#if !inThread && canStartThread}
                        <MenuItem on:click={initiateThread}>
                            <span class="thread" slot="icon">🧵</span>
                            <div slot="text">{$_("thread.menu")}</div>
                        </MenuItem>
                    {/if}
                {/if}
                {#if canForward && !inThread && !inert}
                    <MenuItem on:click={forward}>
                        <ForwardIcon
                            size={$iconSize}
                            color={"var(--icon-inverted-txt)"}
                            slot="icon" />
                        <div slot="text">{$_("forward")}</div>
                    </MenuItem>
                {/if}
                {#if confirmed && groupChat && !me && !inThread && !isProposal && !inert}
                    <MenuItem on:click={() => dispatch("replyPrivately")}>
                        <ReplyOutline
                            size={$iconSize}
                            color={"var(--icon-inverted-txt)"}
                            slot="icon" />
                        <div slot="text">{$_("replyPrivately")}</div>
                    </MenuItem>
                    {#if canBlockUser}
                        <MenuItem on:click={blockUser}>
                            <Cancel
                                size={$iconSize}
                                color={"var(--icon-inverted-txt)"}
                                slot="icon" />
                            <div slot="text">{$_("blockUser")}</div>
                        </MenuItem>
                    {/if}
                {/if}
                {#if canEdit && !inert}
                    <MenuItem on:click={() => dispatch("editMessage")}>
                        <PencilOutline
                            size={$iconSize}
                            color={"var(--icon-inverted-txt)"}
                            slot="icon" />
                        <div slot="text">{$_("editMessage")}</div>
                    </MenuItem>
                {/if}
                {#if (canDelete || me) && !crypto && !inert}
                    <MenuItem on:click={deleteMessage}>
                        <DeleteOutline
                            size={$iconSize}
                            color={"var(--icon-inverted-txt)"}
                            slot="icon" />
                        <div slot="text">
                            {$_(me ? "deleteMessage" : "deleteMessageAndReport")}
                        </div>
                    </MenuItem>
                {/if}
                {#if canUndelete}
                    <MenuItem on:click={undeleteMessage}>
                        <DeleteOffOutline size={$iconSize} color={"var(--icon-txt)"} slot="icon" />
                        <div slot="text">{$_("undeleteMessage")}</div>
                    </MenuItem>
                {/if}
                {#if translatable}
                    {#if translated}
                        <MenuItem on:click={untranslateMessage}>
                            <TranslateOff
                                size={$iconSize}
                                color={"var(--icon-inverted-txt)"}
                                slot="icon" />
                            <div slot="text">{$_("untranslateMessage")}</div>
                        </MenuItem>
                    {:else}
                        <MenuItem on:click={translateMessage}>
                            <TranslateIcon
                                size={$iconSize}
                                color={"var(--icon-inverted-txt)"}
                                slot="icon" />
                            <div slot="text">{$_("translateMessage")}</div>
                        </MenuItem>
                    {/if}
                {/if}
            </Menu>
        </div>
    </MenuIcon>
</div>

<style type="text/scss">
    .menu {
        $offset: -2px;
        position: absolute;
        top: -4px;
        right: $offset;

        &.rtl {
            left: $offset;
            right: unset;
        }
    }

    .menu-icon {
        transition: opacity ease-in-out 200ms;
        opacity: 0;
    }
</style>