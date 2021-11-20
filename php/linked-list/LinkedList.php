<?php

declare(strict_types=1);

class Node
{
    function __construct(
        public int $data,
        public ?Node $prev = null,
        public ?Node $next = null
    ) {
    }
}

class LinkedList
{
    private ?Node $head = null;
    private ?Node $tail = null;

    // Add after tail
    public function push(int $data)
    {
        $node = new Node($data);

        // List is empty
        if ($this->head === null) {
            $this->head = &$node;
        } else { // List has at least one element
            $node->prev = &$this->tail;
            $this->tail->next = &$node;
        }

        $this->tail = &$node;
    }

    // Remove from tail
    public function pop(): int | null
    {
        // List is empty
        if ($this->tail === null) return null;

        // List has one element
        if ($this->tail->prev === null) {
            $result = $this->head;
            $this->head = null;
            $this->tail = null;
            return $result->data;
        }

        // List has two or more elements
        $old_tail = $this->tail;
        $this->tail = &$old_tail->prev;
        $this->tail->next = null;
        return $old_tail->data;
    }

    // Remove from head
    public function shift(): int | null
    {
        // List is empty
        if ($this->tail === null) return null;

        // List has one element
        if ($this->head->next === null) {
            $result = $this->head;
            $this->head = null;
            $this->tail = null;
            return $result->data;
        }

        // List has two or more elements
        $old_head = $this->head;
        $this->head = &$old_head->next;
        $this->head->prev = null;
        return $old_head->data;
    }

    // Add before head
    public function unshift(int $data)
    {
        $node = new Node($data);

        // List is empty
        if ($this->head === null) {
            $this->tail = &$node;
        } else { // List has at least one element
            $this->head->prev = &$node;
            $node->next = &$this->head;
        }

        $this->head = &$node;
    }
}
