# SettleTown – Game Design Document (v0.1)

```table-of-contents
```

## Overview

**SettleTown** is a cozy simulation and town-building game set in the Old West. Players control an individual settler working to establish and grow a township. Through building structures, managing governance, running businesses, enforcing (or bending) the town's laws, and interacting with a cast of non-player characters (NPCs), players shape the story and character of their town, culminating in a final "reckoning" event that the player must deal with in cooperation with the entire town.

### Elevator Pitch
**SettleTown** is a cozy yet challenging town-building simulation set in the mythic Old West.
Starting with nothing but a wagon and a dream, players forge a frontier settlement — digging wells, raising buildings, managing settlers, and shaping the town's destiny through choices that echo across generations.

Behind the dusty trails and warm campfires, however, ancient mysteries stir. As prosperity grows, players must confront the supernatural forces hidden beneath the land or risk seeing their town swallowed by legend.

No two settlements — or legacies — will ever be the same.

### Tagline
> Build your future, weather the frontier, and uncover the legends buried beneath the dust.

### Genre, Style, and Inspirations
**SettleTown** is a cozy frontier simulation with survival, town-building, and light narrative elements, set against an Old West backdrop tinged with folklore and mystery.

**Genre Blend**:
- Cozy Life Sim
- Town Builder
- Light Survival Management
- Narrative Sandbox with Supernatural Mystery

**Tone and Atmosphere**:
- Inviting but occasionally bittersweet.
- A balance between rugged frontier hardships and the warmth of a growing community.
- Subtle, eerie elements inspired by folk tales and western legends — avoiding full horror while embracing mythic strangeness.

**Key Inspirations**:
- **_Stardew Valley_**: Character-driven community building, meaningful relationships, satisfying daily gameplay loops.
- **_Sid Meier’s Colonization_**: The challenge of establishing a new settlement, managing resources, and adapting to an evolving world.
- **_Age of Empires_**: Expansion, specialization, and interactions with dynamic units and environments.
- **Oregon Trail**: The original frontier strategy game. Much of the lore of the old west will come from this game, and there will be subtle nods to it over the course of the storyline.
- **Graveyard Keeper**: This morbid fantasy game gave inspiration in terms of visual style, controls, some game mechanics, and audio.
- **Old West Folklore and Mythology**: Legends of cursed mines, ghost towns, traveling snake oil salesmen, and the mysterious "wildness" beyond civilization.

**Visual Style**:
- **Highly detailed 2D pixel art**, similar in quality and richness to **_Graveyard Keeper_**.
- Full, vibrant color palette emphasizing natural landscapes (rolling plains, dense forests, arid deserts).
- **Handmade wooden structures** dominate the architectural style, reflecting the resourcefulness and craftsmanship of frontier life.
- Subtle environmental effects — dust storms, rain showers, heat haze — enhance immersion without overwhelming the cozy aesthetic.

### Visual and Mood Inspirations

**Mood Board Words**:
- Weathered wood
- Dusty crossroads
- Prairie winds
- Creaking saloon signs
- Fading campfires
- Golden wheat fields
- Thundercloud horizons
- Cracked leather boots
- Lantern glow
- Slow-rolling tumbleweeds
- Shifting desert sands
- Rusted iron fences
- Howling coyotes at dusk
- Ghost lights over the hills
- Whispered folk songs
- Broken wagon wheels
- Hollow mine shafts
- Faded wanted posters
- Whispered legends
### Feature Pillars
- **Cozy Survival**: Blend personal hardship and community growth in an Old West setting.
- **Dynamic Town Building**: Shape a living, breathing frontier town with meaningful NPC relationships.
- **Permanent Choices**: Single-life characters with impactful consequences.
- **Mystery and Myth**: Uncover supernatural secrets beneath the land.
- **Legacy Gameplay**: Each settlement leaves its mark for future games.

### Target Audience
SettleTown aims to attract a diverse group of players who appreciate depth, narrative, and a blend of familiar and unique gameplay mechanics. The primary target audiences include:

- **Cozy Gamers & Life Simulation Enthusiasts:**
    - **Demographics:** This group skews towards female players and can include a wide age range, from young adults to older gamers. They are often looking for relaxing, stress-free, and emotionally engaging experiences. Many in this group also appreciate games that can be played at their own pace. (Inspired by Stardew Valley, cozy game trends)
    - **Motivations:** Players seeking a calming escape, the satisfaction of nurturing growth (both of a town and relationships), character-driven stories, and a sense of community. They will be drawn to SettleTown's detailed pixel art, the warmth of a growing community, relationship-building with NPCs, and the satisfaction of the daily gameplay loops (foraging, building, simple crafting). The "cozy survival" aspect, which emphasizes personal hardship balanced with community growth, will appeal to those who enjoy a touch of challenge without overwhelming stress.
- **Town-Building & Strategy Simulation Players:**
    - **Demographics:** This audience can span from teenagers to adults, often with an interest in strategic depth, management, and creative expression. While traditionally a male-dominated genre, town-builders are increasingly attracting a broader audience, especially on PC and mobile platforms.
    - **Motivations:** These players enjoy the process of creation, optimization, and seeing their decisions shape a larger entity. They are interested in resource management, economic systems, and the challenge of establishing and expanding a settlement. SettleTown’s features like managing settlers, constructing various buildings (Tavern, Sheriff Station, Bank), specializing the town's economy (ranching, mining, trade), and dealing with external threats will be key attractions. Inspirations like _Sid Meier’s Colonization_ and _Age of Empires_ resonate here, appealing to those who like to plan, build, and manage an evolving world.
- **Players Interested in Narrative Sandbox & Light Mystery:**
    - **Demographics:** This group is often composed of players who enjoy exploration, lore, and uncovering secrets. The age range can be broad, typically from teenagers to adults who appreciate storytelling within their gameplay.
    - **Motivations:** Driven by curiosity and a desire for a game world with depth beyond surface mechanics. They will be intrigued by the Old West folklore, the subtle, eerie supernatural elements, and the culminating "Reckoning" event. The idea of permanent choices with impactful consequences, the legacy gameplay where each settlement leaves its mark, and the potential to "uncover legends buried beneath the dust" will be strong hooks. The influence of _Graveyard Keeper_'s slightly darker, mysterious tone combined with the narrative sandbox approach will appeal to this group.
- **Fans of Old West & Frontier Themes:**
    - **Demographics:** This audience often includes players interested in historical settings, adventure, and the unique challenges and aesthetics of the American frontier. This can attract history enthusiasts and those who enjoy the romanticized, rugged individualism of the Old West.
    - **Motivations:** Immersion in a distinct historical and mythological setting. They will appreciate the visual style (weathered wood, dusty crossroads, handmade structures), the thematic elements (saloons, sheriffs, outlaws), and the nods to _Oregon Trail_ and Old West folklore. The game’s tagline, "Build your future, weather the frontier, and uncover the legends buried beneath the dust," directly speaks to this audience.
    - **Notes**: If we were to add in relations to historical events that actually happened, such as the expansion of the railroad system, or specific markers in the Civil War, we might be able to attract more of this demographic. Similar to those attracted to the Assassin's Creed series, there will be history fans that are engaged by our viewpoint of actual historical events. If we took this route, we would need to be careful not to make this a deviation from our actual goals of creating a self-contained narrative, though.
- **Light Survival Management Fans:**
    - **Demographics:** Players who enjoy a degree of challenge related to resource management and environmental factors, but without the hardcore intensity of pure survival games.
    - **Motivations:** The satisfaction of overcoming obstacles and ensuring the well-being of their character and community. SettleTown’s mechanics of managing hunger, weather, and a day/night cycle, especially in the initial "Start Phase," cater to this, offering a sense of accomplishment without being overly punishing, fitting the "cozy yet challenging" descriptor.

**Secondary Audience Considerations:**
- **Indie Game Enthusiasts:** Players who actively seek out unique, passion-driven projects, often appreciating pixel art aesthetics and innovative genre blends.
- **Players Seeking High Replayability:** The "Permanent Choices" and "Legacy Gameplay" feature pillars are designed to attract players who value games where different playthroughs offer unique experiences and consequences.
- **Role-players:** The diverse character archetypes and the ability to shape the town's destiny through player actions will appeal to those who enjoy embodying a character and influencing a dynamic world.

By targeting these overlapping player groups, SettleTown aims to cultivate a dedicated community that appreciates its blend of cozy gameplay, strategic town management, and unfolding frontier mystery. The initial PC (Steam) release aligns well with the preferences of strategy, simulation, and indie game players, with future platform expansions intended to broaden its reach to a more casual console audience.
### Platform
Initially, the game will target PC players via the Steam platform. This is partially because the language we have chosen to use is _Rust_. Mostly this is for its memory safety, strong typing, and ease of use across several platforms (Linux, Windows, and MacOS). Because we are using Bevy, a Rust-based game engine, we are able to also target Android platforms with little to no modifications to the base game. iOS will be the first port, depending on how much work needs to be done to accomplish moving to that platform.

As popularity grows, we intend to port the game to console platforms such as XBox, Switch 2, and PlayStation. This could mean rewriting significant portions of the game in C#, or even using a different game engine as time goes on. Since the focus of the game is to reach casual players, it should be independent of platform. Our support of the Rust ecosystem is based on knowledge of the development team as of the writing of this document. As the team grows, we may have more expertise elsewhere, which will enable us to shift the game to something like Unity or Unreal.

### Visual Style
    - Concept art notes, general look (2D? 3D? Top-down? Isometric?), art references.

### Audio Style
    - Music tone (acoustic? western frontier?), sound FX (creaky doors, cicadas at night).

### Character Archetypes
This is a list of characters that can be used as the primary "archetypes," or templates for player characters. The goal is to have the player be able to create their own character from scratch, and so these might serve as examples in the future. However, in the first phase of the project, these will be the playable characters to choose from.

#### BadlandsBart
- **Concept:** Bart is the embodiment of blue-collar grit, a man who's seen his share of hard knocks and learned that survival often means walking a fine line. He’s not a criminal, but a pragmatist forged by circumstance, willing to bend the rules and work tirelessly to carve out a life when the straight and narrow has proven a dead end. He's about getting the job done, whatever it takes, without illusions of grandeur.
- **Core Gameplay Focus:** Excels at manual labor, resourcefulness, and making the most out of scarce opportunities. He's adept at the "light survival management" aspect, finding ways to stretch resources and endure hardships. His "shady" side might manifest in an ability to find unconventional solutions or deal with less reputable elements for necessary goods or information, without necessarily attracting overt trouble.
- **Potential Starting Benefits/Skills:**
    - **Hardy Constitution:** Starts with slightly better resilience to hunger, thirst, or fatigue, reflecting a life of toughing it out.
    - **Scrounger's Eye:** An innate talent for finding a little extra when gathering basic resources (e.g., an extra piece of wood, slightly more yield from a rough patch of land).
    - **Pragmatic Crafting:** May have a few unique recipes for improvised tools or items made from common materials, or a knack for making basic repairs with fewer supplies.
    - **Streetwise Barter:** While not a smooth talker for big deals, Bart might get slightly better terms on basic necessities from certain types of traders, or know how to acquire essential goods through "alternative" channels when official ones are dry.
    - **Efficient Labor:** Could have a slight speed bonus to common labor tasks like digging, chopping, or basic construction, born from a lifetime of hard work.
- **Motivations/Backstory Hooks:**
    - Likely comes from a background of honest work that didn't pay off, perhaps a failed farm due to bad weather or a swindled partnership.
    - He's in SettleTown for a fresh start, but with realistic expectations – he's not looking to strike it rich, just to build something stable and not starve.
    - Might have debts to pay or family elsewhere he's trying to support, adding urgency to his need to "get by."
    - His "victim of circumstances" past means he's wary but not necessarily unkind; he understands struggle.
- **Connection to GDD Elements:**
    - Deeply connected to the "Cozy Survival" and "Light Survival Management" pillars.
    - His willingness to "bend the town's laws" could lead to unique interactions or opportunities, perhaps even an early understanding of the town's undercurrents.
    - Interactions with "Outlaw" or "Gambler" NPC types might be different for him – perhaps more understanding or more wary due to past experiences.
    - During "The Reckoning," Bart wouldn't be looking for heroic glory or mystical solutions, but rather the most practical, down-to-earth ways to protect the town's essential resources and ensure its people can endure the upheaval. He's the one who'd figure out how to reinforce the well against supernatural contamination using just what's on hand.

BadlandsBart is a character who will resonate with players who appreciate a story of resilience and quiet determination against the odds. He offers a grounded perspective in a world tinged with myth and mystery.

#### BankerBill![[BankerBillConcept.png]]
- **Concept:** BankerBill arrives in SettleTown not with calloused hands, but with a hefty coin purse and an eye for opportunity. He believes capital is the true pioneer, and personal labor is a last resort. Greedy, a touch lazy, and certainly corruptible (especially with promises of more wealth), Bill is not an outright criminal but operates shrewdly within the flexible ethics of the frontier. He prefers to invest, influence, and employ rather than engage in the sweat and toil of settlement building himself.
- **Core Gameplay Focus:** Leveraging his significant starting capital to accelerate development, establish profitable enterprises, and influence the town's economy for his personal benefit. He will focus on hiring other settlers for labor, constructing financial and commercial buildings, and potentially engaging in "legal but ethically dubious" schemes to expand his fortune.
- **Potential Starting Benefits/Skills:**
    - **Venture Capital:** Starts with a significantly larger sum of money than other archetypes, and perhaps a few valuable assets or bonds he can liquidate.
    - **Hired Help Ledger:** May have early or unique access to a system for hiring NPCs to perform tasks (gathering, construction, etc.), reflecting his preference for delegation. _Alternatively, NPCs might be more willing to work for him if he offers a slight wage premium._
    - **Resource Broker:** Can use his funds to order shipments of specific resources from "back East" from the outset, bypassing some initial scarcity (though perhaps at a premium cost that only he can comfortably afford).
    - **Financial Leverage:** Once a Bank is established (likely by him), he may gain better interest rates, offer more complex financial products (loans to NPCs or the town with terms favorable to him), or have a keener sense for spotting profitable ventures.
    - **Debilitating Laziness:** To balance his resources, Bill might have a significant penalty to his own stamina or speed when performing manual labor, strongly incentivizing him to pay others.
- **Motivations/Backstory Hooks:**
    - Sees the untamed frontier as an unregulated marketplace, ripe for exploitation by a man of his financial acumen and "flexible" morals.
    - Perhaps he's looking to establish a new financial empire away from the saturated markets (and stricter regulations) of the East.
    - Could be escaping a minor scandal or "aggressive" business rivals, seeking a place where his wealth can quickly make him influential.
    - His primary goal is to establish a comfortable and profitable life with minimal personal exertion, letting his money do the work.
- **Connection to GDD Elements:**
    - **Dynamic Town Building:** He'll try to shape the town into a hub of commerce that benefits him, prioritizing buildings like the "Bank," "General Store," and "Tavern."
    - **NPC Settler Types and Behavior:** He will actively try to attract (and employ) settlers like Merchants or Laborers. His interactions with NPCs will be transactional; he might offer better pay but expect unquestioning service. His "corruptible" nature means he might be approached by or seek out "Gambler" or even "Outlaw" types for certain tasks.
    - **Economy System & Economic Specialization:** He will be a prime mover in the town's economy, pushing it towards trade or other lucrative specializations. He might create situations of high reward but also high risk for the town.
    - **"Bending the town's laws":** He'd do this through financial influence, "generous donations" to sway decisions, or by creating economic dependencies.
    - **Supernatural Reckoning:** BankerBill would likely view the Reckoning as a threat to his assets. His response would be to fund defenses (built by others), hire specialists (perhaps even the "Snake Oil Salesman" if they claim to have a solution), or attempt to buy off or find a financially viable "solution" to the supernatural threat. If it becomes a bad investment, he might be the first to cut his losses and advocate for abandoning the town to save himself and his liquid assets.

BankerBill introduces a playstyle focused on management, delegation, and economic power, offering a contrast to the more hands-on, community-minded, or survival-focused archetypes. He could be a source of both prosperity and tension within SettleTown.
#### DoctorDaniel
- **Concept:** DoctorDaniel is a dedicated healer, committed to the well-being of others in the often-unforgiving frontier. He carries his medical knowledge and a strong ethical compass, believing in doing right by people even when it's difficult. He’s not afraid of hard work or the grim realities of frontier medicine ("getting his hands dirty") but firmly draws the line at criminal activities or compromising his principles for expediency.
- **Core Gameplay Focus:** Healing the player and other settlers, crafting medicinal supplies from local herbs and scarce resources, improving overall town health and morale, and potentially researching new ailments or remedies unique to the region. He'll be a pillar of support and a voice of reason in the community.
- **Potential Starting Benefits/Skills:**
    - **Physician's Bag:** Starts with a basic kit containing bandages, a simple antiseptic, and perhaps a medicinal tincture or two.
    - **Trained Eye:** More adept at diagnosing illnesses or injuries, possibly revealing specific needs or vulnerabilities in afflicted characters.
    - **Herbalist's Primer:** Begins with knowledge of several local medicinal herbs and basic healing recipes, and may gather them more effectively.
    - **Calming Presence:** Injured or sick settlers under his care (or in a town where he's influential) might recover slightly faster or have a lower chance of complications. NPCs may have a slightly higher initial trust in him.
    - **Sanitary Practices:** May subtly improve hygiene around his initial dwelling or offer early insights into preventing common sicknesses, benefiting the early settlement.
- **Motivations/Backstory Hooks:**
    - Driven by a profound sense of duty to alleviate suffering, perhaps after witnessing loss or hardship that could have been prevented with medical care.
    - May have left a more established practice in the East, seeking a place where his skills are more urgently needed or where he can practice medicine free from bureaucracy.
    - Could be interested in the unique flora of the frontier for its medicinal potential or be tracking a particular disease or condition.
    - He values life and health above profit, putting him at odds with purely exploitative ventures.
- **Connection to GDD Elements:**
    - **Cozy Survival & Light Survival Management:** Directly mitigates threats like sickness and injury for the player and NPCs.
    - **Dynamic Town Building & NPC Relationships:** His skills make him invaluable to settlers, fostering strong relationships and loyalty. He would naturally support the establishment of a clinic or apothecary.
    - **NPC Occupations:** As the "Doctor," he is a cornerstone of a thriving community.
    - **Supernatural Reckoning:** When faced with strange illnesses or injuries caused by supernatural events, DoctorDaniel would initially seek rational explanations. His commitment to healing would force him to confront the unknown, perhaps leading him to research unconventional cures or protective measures, always striving for ethical solutions even in desperate times. His aversion to shady dealings might create conflict if others propose morally ambiguous paths to survival.

DoctorDaniel offers a playstyle centered on care, knowledge, and ethical community building, providing a strong contrast to characters who might operate in moral grey areas. He'd be essential for the long-term health and stability of SettleTown.

#### EntrepreneurEllie
- **Concept:** Ellie is the hard-working heart of SettleTown's commerce, the proprietor of the essential General Store that keeps residents and travelers supplied. She believes in fair dealing, honest labor, and the power of a well-stocked shelf to build a community. While generally law-abiding, Ellie has little patience for bureaucratic red tape that stifles practical enterprise or common sense. She's not afraid to "creatively interpret" or even outright ignore laws she deems unjust or pointlessly obstructive, especially if it benefits the town. More than just a shopkeeper, Ellie is often the unassuming glue holding the community together, a trusted confidante, and a surprisingly influential figure whose support can be invaluable.
- **Core Gameplay Focus:**
    - Establishing and managing a successful General Store, sourcing goods, managing inventory, and meeting the diverse needs of the settlers.
    - Building strong, positive relationships with all community members through reliable service and fair prices. Her store is a natural information hub.
    - Serving as a central point for local trade, perhaps organizing local supply chains or fulfilling special orders for townsfolk.
    - Occasionally navigating or "bending" bureaucratic rules (e.g., bypassing cumbersome regulations to secure vital goods, finding alternative suppliers when official channels are clogged) for the practical benefit of her business and the town.
- **Potential Starting Benefits/Skills:**
    - **Shopkeeper's Ledger:** Starts with a modest but well-chosen inventory for her store and perhaps a small amount of seed capital.
    - **Keen Barter:** Slightly better buy/sell prices when trading with external merchants or a knack for striking favorable bulk deals.
    - **Community Pulse:** Gains a positive reputation with settlers more quickly. NPCs might more readily share local news, needs, or express demands for specific goods at her store, potentially unlocking quests or trade opportunities.
    - **Resourceful Sourcing:** Has an occasional unique option to "creatively acquire" or "expedite" shipments of goods that might be scarce or held up by "red tape," without attracting major negative consequences if handled discreetly.
    - **Trusted Ally:** Once a strong relationship is formed, Ellie might offer discounts, access to rare goods, or provide unique, beneficial quests or town-wide boons due to her extensive network and practical problem-solving skills.
- **Motivations/Backstory Hooks:**
    - Driven by a genuine desire to build a thriving, self-sufficient community where people can prosper through honest work.
    - May have experienced firsthand how overly restrictive regulations or corrupt officials can cripple a town and is determined to protect SettleTown from such fates.
    - Finds satisfaction in providing for others and being a central, reliable figure in the town's daily life.
    - She believes that practical needs and community well-being should always trump arbitrary rules.
- **Connection to GDD Elements:**
    - **General Store:** Her primary domain and a vital piece of "Town Infrastructure." Its success is directly tied to hers.
    - **Economy System & NPC Needs:** She is a cornerstone of the local economy, facilitating the flow of goods and directly addressing settler needs.
    - **NPC Relationship System:** Her role as a central, trusted merchant allows her to build wide-ranging, positive relationships, making her a key social anchor.
    - **"Bending the town's laws":** Her unique form of rebellion is targeted, pragmatic, and often community-oriented rather than self-serving. This could lead to interesting choices when dealing with external authorities or internal town policies.
    - **Town Cohesion & Morale:** A well-run store under Ellie significantly boosts town morale and stability. She might act as an informal mediator or community organizer.
    - **Supernatural Reckoning:** Ellie would be the pragmatic rock, focusing on ensuring the town has the essential supplies to endure. She’d be organizing rationing, seeking alternative supply lines if the main ones are disrupted by supernatural events, and her store would become a beacon of resilience and practical hope. Her willingness to bend rules might be crucial in securing rare or restricted supplies needed to combat supernatural threats.

EntrepreneurEllie  will be a character many players connect with, offering a playstyle centered on building, nurturing community through commerce, and pragmatic problem-solving. Her principled defiance adds an interesting layer to her otherwise steadfast and supportive nature.

#### KillerKate
- **Concept:** KillerKate is an unapologetic criminal, driven purely by self-interest and a belief in survival of the fittest – with herself at the top of that food chain. She views laws as inconveniences, morality as a weakness, and other people as tools or obstacles. Taking a life is merely a means to an end. She thrives on conflict and is loyal to no one but herself, making her a volatile and dangerous presence in any settlement. She isn't in SettleTown to build a community, but to exploit it, rule it through fear, or bleed it dry.
- **Core Gameplay Focus:**
    - Excelling in combat (both armed and unarmed), intimidation, and stealth.
    - Engaging in criminal activities: robbery, extortion, perhaps even assassination or inciting gang warfare if the systems support it.
    - Undermining law and order, potentially seeking to control the town through fear or by eliminating opposition.
    - Attracting other undesirable elements (Outlaws, desperate individuals) while actively driving away or preying upon more peaceful or principled settlers.
    - Her contributions to the "town" are parasitic; any "building" she does is to fortify her own position or create illicit enterprises.
- **Potential Starting Benefits/Skills:**
    - **Predator's Edge:** Starts with a decent weapon (e.g., a well-maintained revolver or a bowie knife) and enhanced proficiency (e.g., faster attacks, higher critical hit chance, or bonus to intimidation when armed).
    - **Shadow Walker:** Bonus to stealth, making it easier to avoid detection when trespassing or preparing an ambush.
    - **Iron Will, Empty Conscience:** Resists fear or morale penalties from committing violent or criminal acts. May even gain a temporary bonus (e.g., adrenaline, focus) in combat situations or after a successful "heist."
    - **Intimidating Aura:** NPCs (especially non-combatants or those with timid traits) are more susceptible to her demands, threats, or attempts at extortion. Some may flee on sight if her reputation is fearsome enough.
    - **Eye for Weakness:** Can more easily identify vulnerable targets for robbery or NPCs who can be easily manipulated through fear.
- **Motivations/Backstory Hooks:**
    - A history steeped in violence and betrayal, possibly a former gang member who struck out on her own (or eliminated her rivals).
    - May be on the run from a significant bounty or a powerful enemy, seeing SettleTown as a temporary hideout or a new territory to claim.
    - Driven by a nihilistic worldview or a traumatic past that convinced her power and ruthlessness are the only things that matter.
    - Her goal isn't just wealth, but dominance and the freedom to act without consequence. She enjoys instilling fear.
- **Connection to GDD Elements:**
    - **Sheriff Station/Law Enforcement:** Kate's primary antagonist within the town structure. She might aim to destroy it, corrupt its officers, or install her own "law."
    - **NPC Relationship System:** Her relationships will be primarily negative, based on fear, coercion, or temporary, untrustworthy alliances. She will be a constant source of conflict.
    - **NPC Settler Types:** She will attract "Outlaws" and frighten away "Preachers," "Doctors," and law-abiding "Merchants" (unless they pay her protection). Her presence will drastically shift the settler demographics.
    - **Reputation System:** The town's reputation will plummet towards "Lawless Outpost" or "Tyrant's Stronghold." This will affect what kind of travelers and events the town encounters.
    - **"Bending the town's laws":** Kate doesn't bend laws; she breaks them with impunity and expects others to follow _her_ rules.
    - **Dynamic Events:** She is likely to trigger events related to crime waves, vigilante justice (against her), or even interventions from outside law agencies if her notoriety grows too large.
    - **Supernatural Reckoning:** Kate would approach the Reckoning with cold pragmatism and self-preservation. If the supernatural entity can be fought, she'll fight it if it threatens _her_. If it can be bargained with using someone else's life or soul, she'll make the deal. She might even see the chaos as an opportunity to consolidate power or eliminate remaining opposition. The town's collective survival is secondary to her own.

Kate introduces a "villain" playthrough. Her success would mean a very different kind of "thriving" SettleTown – one steeped in fear and violence. This archetype would directly challenge all other community-focused gameplay loops and characters. It would be a true test of the game's systems to accommodate such a disruptive force while still providing a compelling (though dark) gameplay experience.
#### LaceLavinia
- **Concept:** Lavinia is a master of manipulation, a woman who wields charm, information, and the art of persuasion as her primary tools. Her domain is likely a popular Saloon, a "respectable" Boarding House with private rooms, or a similar establishment where secrets are currency and desires are indulged. She is acutely aware of the loneliness and ambition on the frontier and uses it to her advantage. While not one for outright crime or getting her own hands dirty with physical labor, she is deeply corruptible by opportunities for wealth and power, and exceptionally skilled at corrupting others to serve her ends. She aims to build an empire of influence and luxury, one whispered secret and artful suggestion at a time.
- **Core Gameplay Focus:**
    - Establishing and managing a profitable social hub (Saloon, high-end Boarding House, etc.) that attracts a wide range of settlers.
    - Excelling at information gathering, uncovering NPC secrets, desires, and weaknesses through dialogue, observation, and her network.
    - Using manipulative dialogue options, charm, or carefully offered "favors" to influence NPCs, bend town rules, or gain economic advantages.
    - Building a network of loyal staff and "clients" who might provide her with resources, information, or undertake tasks she deems beneath her.
    - Her "laziness" is a disdain for physical toil; her "work" is intellectual and social maneuvering.
- **Potential Starting Benefits/Skills:**
    - **Proprietor's Seed:** Starts with a modest sum of money and perhaps the basic plans or a "deed" to establish her initial social venue (e.g., a small but well-located Saloon or a house suitable for "lodging").
    - **Silver Tongue:** Significant bonus to persuasion, deception, and insight-gathering dialogue options. May unlock unique conversation paths.
    - **Web of Whispers:** NPCs are more likely to share gossip or rumors with her or her staff. She might gain a unique "Influence" or "Secret" resource.
    - **Alluring Demeanor:** Certain NPC types might be more easily swayed by her requests or more willing to offer her favorable terms in business dealings.
    - **Discreet Networker:** Can subtly task her staff (if she employs NPCs in her establishment) to gather information or spread rumors on her behalf.
- **Motivations/Backstory Hooks:**
    - Escaped a past where she was powerless or exploited, now determined to be the one pulling the strings.
    - Sees the frontier as a place where ambition and cunning, especially for a woman, can bypass traditional barriers to power and wealth.
    - Driven by a desire for security, luxury, and control, believing that understanding and manipulating human nature is the surest path.
    - Might be in SettleTown to ensnare a particular wealthy individual, ruin a rival from her past, or simply build the most influential establishment on the frontier.
- **Connection to GDD Elements:**
    - **Tavern/Social Hubs:** This is her power base. She’d prioritize its development and ensure it's the center of town life and information flow.
    - **NPC Relationship System:** Lavinia’s gameplay would revolve around this, building complex, often transactional, and manipulative relationships. She'd know everyone's secrets.
    - **NPC Settler Types:** She would attract and cater to a wide array – from lonely settlers seeking companionship to "Gamblers," "Outlaws" (for information or discreet services), and even seemingly upright citizens with hidden vices or ambitions she can exploit.
    - **"Bending the town's laws":** She would excel at this through subtle influence, blackmail (if pushed), or by ensuring key officials are indebted to her or regular patrons of her "discreet services."
    - **Reputation System:** Her presence could make the town known for its lively (and perhaps risqué) nightlife, attracting certain settlers while potentially dismaying others like DoctorDaniel or a righteous Preacher. The town might gain a reputation as a place where "anything can be arranged."
    - **Supernatural Reckoning:** Lavinia would view the Reckoning as a threat to her comfortable life and established network. She’d use her manipulative skills to try and turn the situation to her advantage: perhaps by directing suspicion onto rivals, convincing others to make necessary "sacrifices," or using the chaos to uncover even deeper secrets. Self-preservation and the preservation of her enterprise would be paramount. She would not be above charming whatever entity she thought she could.

LaceLavinia is a much more active and socially dangerous character, using her wits and charm as her primary weapons and assets. She'd be less about overt financial power like BankerBill and more about the subtle, pervasive power of influence and information that leads to wealth.

#### MarshalMilton
- **Concept:** MarshalMilton is the embodiment of frontier justice—a man of unshakeable principles and a will as strong as the steel of his sidearm. He is strictly law-abiding, believing that order and adherence to the code are the only things separating a settlement from chaos. Tough as nails yet scrupulously fair, Milton is committed to protecting the innocent, punishing the guilty, and forging a town where justice isn't just a word, but a way of life. He doesn't bend the rules; he _is_ the rule.
- **Core Gameplay Focus:**
    - Establishing and upholding the rule of law in SettleTown, likely by founding and managing the Sheriff's office and jail.
    - Actively investigating crimes, pursuing, and apprehending criminals (NPCs or even a player character like Kat Slade).
    - Mediating disputes among settlers with fairness and firm judgment.
    - Working to make SettleTown a safe and secure place, thereby attracting law-abiding citizens and commerce.
    - His gameplay revolves around maintaining high town safety, reducing crime, and fostering a sense of security and order.
- **Potential Starting Benefits/Skills:**
    - **Marshal's Resolve:** Starts with a reliable sidearm and perhaps a symbol of authority (e.g., a tin star) that grants him increased standing in official interactions and a slight intimidation bonus against common ruffians.
    - **Sharp Instincts:** Enhanced ability to detect lies, spot suspicious activity, or notice hidden clues during investigations.
    - **Unyielding Presence:** Highly resistant to bribery and intimidation attempts. Criminal NPCs may be less likely to target him directly unless desperate.
    - **Fair Adjudicator:** When resolving conflicts or delivering judgment, his decisions are more likely to be accepted as fair by law-abiding NPCs, potentially boosting town morale or quickly de-escalating tensions.
    - **Combat Ready:** Possesses solid combat skills, particularly with firearms and perhaps in non-lethal apprehension techniques (if applicable), reflecting training or experience in law enforcement.
- **Motivations/Backstory Hooks:**
    - Possibly a former U.S. Marshal, Texas Ranger, or experienced city policeman who has seen the brutal cost of lawlessness and is determined to prevent it from taking root.
    - May be pursuing a notorious outlaw or gang, and his quest for justice leads him to the burgeoning SettleTown.
    - Driven by a profound, almost zealous belief in the sanctity of the law and the protection of the innocent.
    - Could be seeking redemption for a past failure where he couldn't prevent injustice, making him doubly committed now.
- **Connection to GDD Elements:**
    - **Sheriff Station & Law Enforcement:** This is his natural domain. He would prioritize building it, upgrading it, and potentially recruiting NPCs as deputies (if the system allows).
    - **NPC Relationship System:** He will earn the deep respect and loyalty of law-abiding citizens (like Ellie Merchant or Doctor Daniel). His interactions with criminal elements (like Kat Slade or less scrupulous NPCs) will be inherently adversarial.
    - **NPC Settler Types:** His presence and success will attract settlers who value safety and order (Merchants, families, artisans) and actively deter or lead to the capture/expulsion of "Outlaw" types.
    - **"Bending the town's laws":** Milton stands in stark opposition to this. His strict adherence, even to unpopular or inconvenient laws, can create unique narrative and gameplay conflicts.
    - **Reputation System:** He will strive to build the town's reputation as a bastion of law and order. This could make it a target for significant outlaw gangs looking to make a statement.
    - **Dynamic Events:** He'll be central to events involving crime waves, jailbreaks, posse formation, and delivering justice for wronged NPCs.
    - **Supernatural Reckoning:** Milton would confront the supernatural with a methodical, investigative approach, initially seeking to debunk or find a rational explanation. When faced with undeniable otherworldly threats, his duty to protect the town would remain paramount. He would try to establish organized defenses, maintain morale through calm leadership, and ensure that even in the face of terror, the community does not descend into lawless panic or resort to unjust measures (like random sacrifices). He would view any entity causing harm as a lawbreaker, no matter its origin.

MarshalMilton offers a playthrough focused on order, justice, and community protection through unwavering adherence to the law. He is the archetypal hero lawman, providing a strong moral compass and a direct challenge to the more chaotic or criminal elements within SettleTown. His presence will define the town's character as much as any outlaw's would, but in the opposite direction.

#### SawdustSadie
- **Concept:** Sadie, often called "Sawdust" for her time working rough lumber or simply for her unpolished grit, is a woman carved from the very timber and stone of the frontier. She's no stranger to hard labor and lean times, having learned that survival often means being tougher, craftier, and more resilient than the next person. She's inherently practical, somewhat wary, and not above bending a few rules to get by, though she draws a firm line before outright crime. She values her independence fiercely.
- **Core Gameplay Focus:** Excels at resource gathering, basic construction, and making do with limited supplies. Her gameplay is about self-sufficiency, endurance, and finding ingenious solutions to problems with whatever is at hand. She's a master of the "light survival management" aspect.
- **Potential Starting Benefits/Skills:**
    - **Unyielding Stamina:** Long days of labor have made her more resistant to fatigue; her stamina depletes slower during physical tasks.
    - **Resourceful Salvage:** Higher chance of finding extra or slightly better quality materials when scavenging, dismantling, or even harvesting natural resources.
    - **Improvised Solutions:** Can craft a few unique, practical items from common materials that others might overlook, or can make basic tools/structures last a bit longer.
    - **Keen Observation:** She's learned to read situations and people quickly, perhaps giving her a slight edge in noticing environmental hazards or when an NPC is being less than honest about a simple trade.
    - **Takes Up Little Space:** Perhaps requires slightly less food or basic supplies to maintain well-being initially, used to making rations stretch.
- **Motivations/Backstory Hooks:**
    - May have been widowed, abandoned, or simply struck out on her own after realizing relying on others was a vulnerability.
    - Could be escaping a restrictive past or a situation where she was underestimated and exploited.
    - She seeks a place where her hard work translates directly into her own survival and a measure of security, without owing anyone anything.
    - She might have a hidden soft spot or a specific goal (like finding a lost sibling or saving up for a small, independent claim) that fuels her relentless drive.
- **Connection to GDD Elements:**
    - **Cozy Survival:** She embodies the challenging but ultimately rewarding side of this.
    - **Start Phase:** She'd be incredibly efficient at digging the well, building the initial shelter, and sourcing food.
    - **"Bending the town's laws":** Her methods might include things like trapping on disputed land (if game mechanics allow), finding clever ways around resource restrictions, or engaging in barter that skirts official channels.
    - **NPC Interactions:** She'd be wary but fair. Other hardworking or marginalized NPCs might find a quiet solidarity with her.
    - **Supernatural Reckoning:** Sadie would face the supernatural with stubborn practicality. She'd focus on reinforcing structures, finding mundane ways to counter strange phenomena, and ensuring the community has the basic resources to weather any storm, magical or otherwise. She's a survivor, first and foremost.

SawdustSadie offers a playstyle rooted in resilience, resourcefulness, and rugged independence. She embodies the spirit of the self-reliant pioneer, capable of turning scarcity into sufficiency through hard work and ingenuity. Players choosing Sadie will find a deep satisfaction in mastering the frontier's challenges head-on, building a life from the ground up with their own two hands, and facing the unknown with a practical, unyielding determination. She’s a testament to the quiet strength found in those who have learned to depend on themselves above all else.

---
## Gameplay

### Core Gameplay: Start Phase

At the start of **SettleTown**, the player creates their initial character, choosing to play as either a single individual or the leader of a family unit (patriarch or matriarch). In family mode, other family members are AI-controlled.

After creation, the player starts with a small Conestoga wagon and two oxen on a large, explorable map. Early objectives include:

1. **Digging a Well**: Using wagon tools to access a fresh water source.
2. **Building a House**: Gathering wood, rocks, dirt, and pine tar to construct an initial shelter.
3. **Establishing a Main Street**: Designating a central "high street" for future town development.
4. **Sourcing Food**: Foraging and hunting local flora and fauna to survive.

Once the house is built, players can dismantle the wagon to salvage additional materials, transferring useful supplies into their new home. Successfully completing these steps triggers the **Midgame Phase** as the player names their new town.

#### Survival Mechanics (Start Phase)

Players must manage critical survival needs:
- **Hunger**: Regular food intake is necessary for survival.
- **Weather**: Environmental conditions like rain, wind, and heat impact gameplay.
- **Day/Night Cycle**: Visibility, wildlife behavior, and character stamina change dynamically.

Failing to meet these needs can cause sickness, injury, or death.

_(Note: Seasons with farming cycles and natural disasters are planned for a future expansion.)_

### Core Gameplay: Midgame Phase

Once the initial town is founded, **SettleTown** shifts toward community growth and management.

Key Midgame Features:
- **New Settlers**: NPC travelers may arrive seeking to settle, requiring available housing.
- **Housing Assistance**: Helping new settlers build their homes improves loyalty and relationships.
- **Relationship System**: NPCs develop dynamic relationships with the player and each other, leading to alliances, rivalries, or partnerships.
- **Town Infrastructure**: Public buildings can be constructed:
    - **Tavern**: Boosts settler happiness.
    - **Sheriff Station**: Establishes law enforcement.
    - **General Store**: Enables trade and supplies.
    - **Post Office**: Connects the town to distant areas.
    - **Bank**: Manages finances and town treasury.

Each building unlocks gameplay systems and attracts new settlers.

#### Settler Types and Behavior

Settlers are varied, with distinct skills, personalities, and motivations:

- **Occupations**:
    - Blacksmith, Doctor, Gambler, Outlaw, Merchant, Preacher.

- **Personality Traits**:
    - Traits like honest, ambitious, lazy, secretive, or violent influence behavior.

- **Decision to Settle**:
    - Factors like town safety, available housing, player reputation, and amenities impact whether NPCs stay or move on.

- **Dynamic Events**:
    - Special visitors like gunslingers, tycoons, or snake oil salesmen offer rare challenges or opportunities.

### Core Gameplay: Endgame Phase

#### Expansion, Power, and Mystery

As the town flourishes, players face new challenges:

- **Leadership and Governance**: Players can become mayor or sheriff.
- **Economic Specialization**: Towns can specialize (ranching, mining, trade, outlaw haven).
- **Reputation System**: Town status influences interactions with travelers and neighboring settlements.
- **External Threats**: Rival towns, bandits, and market crashes present dangers.

#### The Supernatural Reckoning

When the town reaches a certain size or development threshold, supernatural elements are introduced:

- **The Reckoning Event**:
    - Strange omens, missing people, failing crops, and ghost sightings.

- **Player Response Options**:
    - **Investigate** the haunted mine.
    - **Protect** the town through rituals or fortifications.
    - **Escape** and abandon the settlement.

- **Consequences**:
    - Success results in a thriving legendary town.
    - Failure leads to the town's destruction or curse.
    - Death is permanent — no respawns.

#### Supernatural Content Tone
- Subtle and eerie, rooted in Old West folklore.
- Time anomalies, ghostly settlers, and cursed artifacts deepen the mythos.
- Exploration of the haunted mine can lead to rare rewards or greater dangers.

Personalized ending screens narrate the fate of the town and character.

### New Game Plus: Legacy System

After completing a game, players unlock a **New Game Plus** mode tied to the previous settlement.

Legacy Features:
- **Ruins of the Old Town**: Discover old buildings, artifacts, and relics.
- **Inherited Reputation**: Past actions influence new settlers' behavior.
- **Environmental Changes**: Fertile lands or cursed wastelands, depending on the town's fate.
- **Family Lineage**: Descendants of the original settlers may appear with quests or bonuses.
- **Unique Starting Options**: Access to heirlooms, maps, cursed items, or resources.

Narrative Hooks:
- Legends about "Old SettleTown" circulate among NPCs.
- Ghosts and hidden lore tie the old and new worlds together.
- Players choose to rebuild, redeem, or abandon their legacy.

### Detailed Gameplay

#### Item/Inventory Management
#### Crafting System
#### Combat System
#### Fishing/Hunting Systems
#### Economy System

#### Controls/User Interface
    - Basic control mapping (keyboard/controller layout).
    - UI/UX sketches (inventory screens, town management screens).

#### Progression System
    - How does the town/player grow over time? (You have this, but could show milestone examples.)
---
## Story and Narrative
    - Basic story structure (even in sandbox games, a light narrative thread helps).
    - Example NPC background stories or town events.
    - Supernatural lore snippets.

---
## Sales and Marketing
    - Premium game? DLC expansions? Cosmetic microtransactions?
    - (Only if needed.)

---
## Development
### Technical Details
    - Engine (Unity, Unreal, custom?)
    - Networking if multiplayer (if you ever want settlers to also be real players someday?).

### Development Roadmap
    - Phases (Prototype → Alpha → Beta → Launch).
    - Priorities for first playable version.