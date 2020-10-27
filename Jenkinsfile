pipeline {
  agent { label 'MacMini' }
  options { skipDefaultCheckout() }
  parameters {
    gitParameter(name: 'GIT_TAG',
                 type: 'PT_BRANCH_TAG',
                 description: 'The Git tag to checkout. If not specified "master" will be checkout.',
                 defaultValue: 'master')
    booleanParam(name: 'PUBLISH_RESULTS',
                 description: 'Publish the resulting wheels to Pypi.org',
                 defaultValue: false)
  }

  stages {
    stage('Checkout Git TAG') {
      steps {
        deleteDir()
        checkout([$class: 'GitSCM',
                  branches: [[name: "${params.GIT_TAG}"]],
                  doGenerateSubmoduleConfigurations: false,
                  extensions: [],
                  gitTool: 'Default',
                  submoduleCfg: [],
                  userRemoteConfigs: [[url: 'https://github.com/eclipse-zenoh/zenoh-python.git']]
                ])
      }
    }

    stage('MacOS wheels') {
      steps {
        sh '''
        . ~/.zshenv
        export PATH=$PATH:~/miniconda3/envs/zenoh-cp35/bin
        export PATH=$PATH:~/miniconda3/envs/zenoh-cp36/bin
        export PATH=$PATH:~/miniconda3/envs/zenoh-cp37/bin
        export PATH=$PATH:~/miniconda3/envs/zenoh-cp38/bin
        maturin build --release
        '''
      }
    }

    stage('Manylinux2010-x64 wheels') {
      steps {
        sh '''
        docker run --init --rm -v $(pwd):/workdir -w /workdir adlinktech/manylinux2010-x64-rust-nightly maturin build --release --manylinux 2010
        '''
      }
    }

    stage('Manylinux2010-i686 wheels') {
      steps {
        sh '''
        docker run --init --rm -v $(pwd):/workdir -w /workdir adlinktech/manylinux2010-i686-rust-nightly maturin build --release --manylinux 2010
        '''
      }
    }

    stage('Manylinux2014-aarch64 wheels') {
      steps {
        sh '''
        docker run --init --rm -v $(pwd):/workdir -w /workdir adlinktech/manylinux2014-aarch64-rust-nightly maturin build --release --manylinux 2014
        '''
      }
    }

    stage('Deploy to download.eclipse.org') {
      steps {
        sshagent ( ['projects-storage.eclipse.org-bot-ssh']) {
          sh '''
          if [ "${PUBLISH_RESULTS}" = "true" ]; then
            ssh genie.zenoh@projects-storage.eclipse.org rm -fr /home/data/httpd/download.eclipse.org/zenoh/zenoh-python/${TAG}
            ssh genie.zenoh@projects-storage.eclipse.org mkdir -p /home/data/httpd/download.eclipse.org/zenoh/zenoh-python/${TAG}
            scp target/wheels/*.whl target/wheels/*.tar.gz genie.zenoh@projects-storage.eclipse.org:/home/data/httpd/download.eclipse.org/zenoh/zenoh-python/${TAG}/
          else
            echo "Publication to download.eclipse.org skipped"
          fi
          '''
        }
      }
    }

    stage('Deploy on pypi.org') {
      steps {
        sh '''
          if [ "${PUBLISH_RESULTS}" = "true" ]; then
            python3 -m twine upload --repository eclipse-zenoh target/wheels/*.whl target/wheels/*.tar.gz
          else
            echo "Publication to Pypi.org skipped"
          fi
        '''
      }
    }
  }

  post {
    success {
        archiveArtifacts artifacts: 'target/wheels/*.whl, target/wheels/*.tar.gz', fingerprint: true
    }
  }
}