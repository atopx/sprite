# sprite
A simple, high-performance, cross-platform automation tool


## example

> sprite examples/script.spr

```spr
************** start script **************

** Set variable a of type pos, pos is a built-in coordinate type with (x, y) attribute, 
** in this case a's x=100 a's y=200
pos a 100 200

pos b 500 500

** start loop, 10 is the number of loops
loop-start 10

    ** move the mouse cursor to variable b(500, 500)
    move b 

    ** click the left mouse button once
    mouse left 

    ** sleep 100ms
    sleep 100

    ** move the mouse cursor to (300, 400)
    move 300 400

    ** click the left mouse button 10 times
    mouse left 10

    ** sleep 1s
    sleep 1000

    ** click the right mouse button 10 times
    mouse right 10

loop-end ** end the loop block
```

## Built keyword

- `**`: code comment
- `pos`: coordinate type with (x, y) attribute
- `move`: move the mouse cursor
- `mouse`: click the mouse `left | right` `tims`, default once
- `sleep`: sleep time millisecond
- `loop-start` -- `loop-end`: loop block, requires an integer parameter to be used as the loop count
