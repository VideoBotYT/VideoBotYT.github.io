<script lang="ts">
    const contributors = [
      { name: "VideoBot", url: "#", description: "The one who literally made everything", iconURL: "/images/VideoBot.png" },
      { name: "Daveberry Cricket Blueson", url: "https://blueson.tech/", description: "The guy who helps too much, and most code are from his website.", iconURL: "https://codedave.pages.dev/assets/images/Daveberry%20Wave.png" }
    ];
</script>

<main>
    <div class="header">
        <h2>The people who conributed</h2>
        <span>Check out the people who contributed on this website. You might be familiar with them.</span>
    </div>

    <div class="contributors">
        {#each contributors as contributor}
            <a href={contributor.url}>
                <article>
                    <img src={contributor.iconURL} alt={contributor.name} width="64" height="64" />
                    <div class="info">
                        <span class="name">{contributor.name}</span>
                        <span>{contributor.description}</span>
                    </div>
                </article>
            </a>
        {/each}
    </div>
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 16px;
    }

    .header {
        text-align: center;
    }

    .contributors {
        display: flex;
        flex-wrap: wrap;
        flex-direction: column;
        gap: 16px;

        article {
            display: flex;
            gap: 8px;

            .info {
                display: flex;
                flex-direction: column;
                gap: 4px;

                .name {
                    font-size: 1.3rem;
                    font-weight: bold;
                }
            }
        }
    }
</style>