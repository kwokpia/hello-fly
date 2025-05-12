````mermaid
graph TD

    subgraph Global_Users ["Global Users"]
        direction LR
        UserIOS
    end

    UserIOS --> GA[AWS Global Accelerator <br> Anycast IPs]

    subgraph AWS_Cloud_Region_A ["AWS Cloud - Region A (e.g., us-east-1)"]
        direction TB
        GA -->|Traffic to Region A| ALB_A[Application Load Balancer A]

        subgraph AppLogic_A ["AppLogic Region A"]
            direction LR
            ALB_A --> APIGW_A[Amazon API Gateway A <br> Optional feature]
            APIGW_A --> Fargate_Service_A[Fargate Service A <br> manages Task1, Task2...]
        end

        Fargate_Service_A --> Cognito[Amazon Cognito <br> Likely single region, or federated]

        Fargate_Service_A --> DB_A_RegionA[Database A - Region A]
        Fargate_Service_A --> DB_B_RegionA[Database B - Region A]
        DB_A_RegionA <--> DB_B_RegionA

        subgraph SharedServices_A ["Shared Services Region A"]
            CloudWatch_A[Amazon CloudWatch A <br> Monitoring & Logging]
        end
        AppLogic_A --> CloudWatch_A
        DB_A_RegionA --> CloudWatch_A
        DB_B_RegionA --> CloudWatch_A
    end

    subgraph AWS_Cloud_Region_B ["AWS Cloud - Region B (e.g., eu-west-1)"]
        direction TB
        GA -->|Traffic to Region B| ALB_B[Application Load Balancer B]

        subgraph AppLogic_B ["AppLogic Region B"]
            direction LR
            ALB_B --> APIGW_B[Amazon API Gateway B <br> Optional feature]
            APIGW_B --> Fargate_Service_B[Fargate Service B <br> manages Task1, Task2...]
        end

        Fargate_Service_B --> Cognito

        Fargate_Service_B --> DB_A_RegionB[Database A - Region B]
        Fargate_Service_B --> DB_B_RegionB[Database B - Region B]
        DB_A_RegionB <--> DB_B_RegionB

        subgraph SharedServices_B ["Shared Services Region B"]
            CloudWatch_B[Amazon CloudWatch B <br> Monitoring & Logging]
        end
        AppLogic_B --> CloudWatch_B
        DB_A_RegionB --> CloudWatch_B
        DB_B_RegionB --> CloudWatch_B
    end

    subgraph Global_Static_Content_and_Auth ["Global Static Content and Authentication/Authorization"]
        direction LR
        UserIOS --> CDN[Amazon CloudFront]
        CDN --> S3[S3 Bucket <br> Static Assets, e.g. UI files]
    end

    subgraph CrossRegion_Data_Replication ["Cross-Region Data Replication"]
        DB_A_RegionA <-->|Replication| DB_A_RegionB
        DB_B_RegionA <-->|Replication| DB_B_RegionB
    end

    %% Note on API Gateway:
    %% The "Optional feature" text for API Gateway indicates that if its primary role was caching/CDN integration,
    %% and GA is now the entry for dynamic traffic, API Gateway might be streamlined or its features handled by ALB/Fargate.
    %% However, if specific API Gateway features (auth, transformation, throttling) are needed *after* ALB, it remains.
    %% A common pattern is GA -> ALB -> Fargate.

    %% Styling
    classDef region fill:#e6f3ff,stroke:#007bff,stroke-width:2px;
    class AppLogic_A,AppLogic_B,SharedServices_A,SharedServices_B region;
    classDef global fill:#e0ffe0,stroke:#006400,stroke-width:2px;
    class GA,Cognito global;
    classDef awsCloudRegion fill:#fff0e6,stroke:#d95f02,stroke-width:2px;
    class AWS_Cloud_Region_A,AWS_Cloud_Region_B awsCloudRegion;
    classDef globalInfra fill:#f5f5f5,stroke:#555,stroke-width:2px;
    class Global_Users,Global_Static_Content_and_Auth,CrossRegion_Data_Replication globalInfra;
    ```
````
