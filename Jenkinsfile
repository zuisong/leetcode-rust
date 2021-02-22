pipeline {
  agent {
    docker {
      image 'rustlang/rust:nightly-alpine'
    }

  }
  stages {
    stage('检出') {
      steps {
        checkout(
            [
                $class: 'GitSCM',
                branches: [[name: env.GIT_BUILD_REF]],
                userRemoteConfigs: [[url: env.GIT_REPO_URL, credentialsId: env.CREDENTIALS_ID]]
            ]
        )
      }
    }
    stage('构建') {
      steps {
        echo '构建中...'
        sh ' echo $PWD  '
        sh ' ls -lR  '
        sh ' cargo test '
        echo ' 构建完成... '
      }
    }
  }
}
