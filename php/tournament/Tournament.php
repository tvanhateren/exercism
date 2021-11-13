<?php

/*
 * By adding type hints and enabling strict type checking, code can become
 * easier to read, self-documenting and reduce the number of potential bugs.
 * By default, type declarations are non-strict, which means they will attempt
 * to change the original type to match the type specified by the
 * type-declaration.
 *
 * In other words, if you pass a string to a function requiring a float,
 * it will attempt to convert the string value to a float.
 *
 * To enable strict mode, a single declare directive must be placed at the top
 * of the file.
 * This means that the strictness of typing is configured on a per-file basis.
 * This directive not only affects the type declarations of parameters, but also
 * a function's return type.
 *
 * For more info review the Concept on strict type checking in the PHP track
 * <link>.
 *
 * To disable strict typing, comment out the directive below.
 */

declare(strict_types=1);

class Team
{
    private $name = "";
    private $wins = 0;
    private $draws = 0;
    private $losses = 0;

    public function __construct($name)
    {
        $this->name = $name;
    }

    public function win()
    {
        $this->wins += 1;
    }

    public function draw()
    {
        $this->draws += 1;
    }

    public function loss()
    {
        $this->losses += 1;
    }

    public function matches_played(): int
    {
        return $this->wins + $this->draws + $this->losses;
    }

    public function points(): int
    {
        return 3 * $this->wins + $this->draws;
    }

    public function as_string(): string
    {
        return sprintf(
            "\n%-30s | %2d | %2d | %2d | %2d | %2d",
            $this->name,
            $this->matches_played(),
            $this->wins,
            $this->draws,
            $this->losses,
            $this->points()
        );
    }
}

class Tournament
{
    const HEADER = "Team                           | MP |  W |  D |  L |  P";

    public function tally(string $matches): string
    {
        $teams = [];
        $matches = preg_split("/;|\n/m", $matches);
        $len = count($matches);

        if ($len === 1) {
            return self::HEADER;
        } else if ($len % 3 !== 0) {
            throw new InvalidArgumentException("Incomplete input, expecting format 'Team A; Team B; <win|lose|draw>'");
        }

        for ($i = 0; $i < $len; $i += 3) {
            $a = $matches[$i];
            $ateam = $teams[$a] ?? new Team($a);

            $b = $matches[$i + 1];
            $bteam = $teams[$b] ?? new Team($b);

            switch ($matches[$i + 2]) {
                case "win":
                    $ateam->win();
                    $bteam->loss();
                    break;
                case "loss":
                    $ateam->loss();
                    $bteam->win();
                    break;
                case "draw":
                    $ateam->draw();
                    $bteam->draw();
                    break;
                default:
                    throw new InvalidArgumentException($matches[$i + 2] . " is not a valid match outcome");
                    break;
            }

            $teams[$a] = $ateam;
            $teams[$b] = $bteam;
        }

        // First sort teams alphabetically
        krsort($teams);
        // Then sort teams associatively by points
        uasort($teams, fn ($a, $b) => $a->points() > $b->points() ? -1 : 1);

        // Map Team objects to string
        $teams = array_map(fn ($team) => $team->as_string(), $teams);

        return self::HEADER . implode($teams);
    }
}
