<script lang="ts">
  export let icon: string = '';
  export let text: string = '';
  export let maxWidth: string = '';
  export let active: boolean;
  export let native: boolean = false;
  export let onClick: Function;
</script>

<div class="outside flex align-center justify-start"
  class:active class:native class:justify-end={active}
  style={maxWidth ? `max-width: ${maxWidth}` : ''}
  on:click={() => onClick()}>
  {#if native}
    <div class="crypto flex align-center justify-center"
      class:active={!active}>
      <i class="jet-icons">
        ❍
      </i>
    </div>
    <div class="usd flex align-center justify-center"
      class:active>
      <i class="jet-icons">
        ❏
      </i>
    </div>
  {:else}
    <div class="toggle flex align-center justify-center">
      <i class="jet-icons"
        class:text-gradient={active}>
        {icon}
      </i>
      {#if text}
        <p class="bicyclette"
        class:text-gradient={active}>
          {text}
        </p>
      {/if}
    </div>
  {/if}
</div>

<style>
  .outside {
    margin: var(--spacing-sm) auto;
    width: calc(100% - 12px);
    background: var(--grey);
    border-radius: var(--round-radius);
    box-shadow: var(--neu-shadow-inset);
    cursor: pointer;
  }
  .outside.active {
    background: var(--gradient);
    box-shadow: var(--neu-shadow-inset-gradient);
  }
  .toggle {
    margin: calc(var(--spacing-xs) / 1.5);
    border-radius: var(--round-radius);
    background: var(--gradient);
  }
  .outside.active .toggle {
    background: var(--white);
  }
  .outside.native, .outside.native.active {
    width: 55px;
    margin: 0 auto;
    padding: unset;
    background: var(--grey);
    border-radius: var(--round-radius);
    box-shadow: var(--neu-shadow-inset-low);
    cursor: pointer;
    overflow: hidden;
  }
  .crypto, .usd {
    width: 50%;
    height: 100%;
    padding: var(--spacing-xs) 0;
  }
  .crypto {
    border-top-left-radius: var(--round-radius);
    border-bottom-left-radius: var(--round-radius);
  }
  .usd {
    border-top-right-radius: var(--round-radius);
    border-bottom-right-radius: var(--round-radius);
  }
  .crypto.active, .usd.active {
    background: var(--gradient);
    box-shadow: var(--neu-shadow);
  }
  .crypto i, .usd i {
    color: var(--black);
    margin: unset;
    font-size: 14px;
    opacity: var(--disabled-opacity);
  }
  .crypto i {
    margin-right: -3px;
  }
  .usd i {
    margin-left: -3px;
  }
  .crypto.active i, .usd.active i {
    color: var(--white);
    opacity: 1;
  }
  i, p {
    padding-left: 1px;
    font-size: 15px;
    line-height: 11px;
    color: var(--white);
  }
  p {
    font-size: 10px;
    text-transform: uppercase;
    padding-top: 2px;
    padding-right: 6px;
    opacity: 1;
  }
</style>