<script lang="ts">
  export let text: string = '';
  export let button: boolean = false;
  export let fullview: boolean = false;
  export let fullscreen: boolean = false;
</script>

<div class="loader flex align-center justify-center column"
  class:button
  class:fullview
  class:fullscreen>
  <div class="outer-circle">
    <div class="inner-circle">
      <img src="img/jet/jet_logomark_gradient.png" alt="Jet Logomark" />
    </div>
  </div>
  {#if text}
    <p class="text-gradient">
      {@html text}
    </p>
  {/if}
</div>

<style>
  .loader {
    left: var(--nav-width) !important;
    top: 0 !important;
  }
  .fullview {
    position: absolute !important;
    width: calc(100vw - var(--nav-width)) !important;
    height: 100vh !important;
  }
  .fullscreen {
    position: absolute !important;
    left: 0 !important;
    width: 100vw !important;
    height: 100vh !important;
  }
  .outer-circle {
    position: relative;
    margin: 10px auto;
    width: 100px;
    height: 100px;
    border-radius: 100% !important;
    box-shadow: inset -2px -2px 6px 0px var(--light-shadow), inset 2px 2px 6px 0px var(--dark-shadow);
  }
  .button .outer-circle {
    width: 31px;
    height: 31px;
    box-shadow: unset;
  }
  .button .inner-circle {
    box-shadow: unset;
  }
  .inner-circle {
    z-index: 2;
    position: absolute;
    top: calc(50% - 40%);
    left: calc(50% - 40%);
    width: 80%;
    height: 80%;
    border-radius: 100% !important;
    background: var(--white);
    box-shadow: -2px -2px 6px 0px var(--light-shadow), 2px 2px 6px 0px var(--dark-shadow),
      inset 0px 0px 0px 0px var(--light-shadow), inset 0px 0px 0px 0px var(--dark-shadow);
  }
  .inner-circle img {
    width: 100%;
    height: auto;
    animation: rotate 1.25s ease-in-out infinite;
  }
  p {
    font-size: 12px;
  }
  @keyframes rotate {
    100% {
      transform: rotate(-360deg);
    }
  }
  @keyframes dash {
    0% {
      stroke-dasharray: 1, 200;
      stroke-dashoffset: 0;
    }
    50% {
      stroke-dasharray: 89, 200;
      stroke-dashoffset: -35px;
    }
    100% {
      stroke-dasharray: 89, 200;
      stroke-dashoffset: -124px;
    }
  }

  @media screen and (max-width: 1100px) {
    .fullscreen {
      height: calc((var(--vh, 1vh) * 100)) !important;
    }
  }
</style>