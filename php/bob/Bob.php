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
class Bob
{
    public function respondTo(string $str): string
    {
        // Remove whitespace before the end of the line
        $str = mb_ereg_replace("\s+$", "", $str);

        $is_quiet = strlen($str) === 0;

        $is_question = str_ends_with($str, "?");

        $is_screaming = mb_convert_case($str, MB_CASE_UPPER) === $str && mb_convert_case($str, MB_CASE_LOWER) !== $str;

        if ($is_quiet) {
            return "Fine. Be that way!";
        } else if ($is_screaming) {
            if ($is_question) {
                return "Calm down, I know what I'm doing!";
            } else {
                return "Whoa, chill out!";
            }
        } else {
            if ($is_question) {
                return "Sure.";
            } else {
                return "Whatever.";
            }
        }
    }
}
