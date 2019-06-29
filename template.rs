/*Helpful resources: 
    https://substrate.dev/recipes/intro/index.html
    https://doc.rust-lang.org/stable/rust-by-example/ */
    
    
    
    /*A function that creates a new pending block based on the previous block's header.
    The header includes: */

        /*The block height;*/

        
        //Code here

        //Test case here


        /*A "cryptographic commitment" to the block's state, this is important for light
        clients to validate that the block is correct. A cryptographic commitment serves
        the same role as a hash, you cannot change the state without invalidating the 
        commitment;*/

        
        //Code here

        //Test case here


        /*A cryptographic commitment to all the extrinsics in the body, which prevents the
        extrinsics from being changed;*/
        

         //Code here

        //Test case here


        /*A hash of the block's parent;*/


        //Code here

        //Test case here


        /*Some extra arbitrary data. One usecase for this data would be for client updates
        - since light clients only sync headers, if you want to update them you can't have
         updates implemented as an extrinsic or the light clients won't receive them.*/
    

         //Code here

         //Test case here


        /*A function that adds an extrinsic (such as a transaction) to a pending block.
        This should also update the chain's state (for example, account balances);*/

        
        //Code here

        //Test case here

    
        /*A function that takes a pending block and generates a finished block from it.
        This finished block can then be propagated throughout the network;*/
    
        
        //Code here

        //Test case here

    
        /*A function that executes an existing block. This is run by full nodes to confirm
        that received blocks are valid before accepting them. For example, in a value-
        bearing chain you could check that no-one tries to transfer more than their balance.*/

    
         //Code here

        //Test case here
